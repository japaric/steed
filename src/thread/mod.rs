#![stable(feature = "rust1", since = "1.0.0")]

use any::Any;
use cell::UnsafeCell;
use ffi::{CStr, CString};
use fmt;
use io;
use str;
use sync::Arc;
use sync::atomic::{AtomicUsize, Ordering};
use sys::thread as imp;
use sys_common::util;
use sys_common::{AsInner, IntoInner};
use time::Duration;

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug)]
pub struct Builder {
    // A name for the thread-to-be, for identification in panic messages
    name: Option<String>,
    // The size of the stack for the spawned thread
    stack_size: Option<usize>,
}

impl Builder {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn new() -> Builder {
        Builder {
            name: None,
            stack_size: None,
        }
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn name(mut self, name: String) -> Builder {
        self.name = Some(name);
        self
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn stack_size(mut self, size: usize) -> Builder {
        self.stack_size = Some(size);
        self
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn spawn<F, T>(self, f: F) -> io::Result<JoinHandle<T>> where
        F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
    {
        let Builder { name, stack_size } = self;

        let stack_size = stack_size.unwrap_or(util::min_stack());

        let my_thread = Thread::new(name);
        let their_thread = my_thread.clone();

        let my_packet : Arc<UnsafeCell<Option<Result<T>>>>
            = Arc::new(UnsafeCell::new(None));
        let their_packet = my_packet.clone();

        let main = move || {
            if let Some(name) = their_thread.cname() {
                imp::Thread::set_name(name);
            }
            unsafe {
                // TODO(steed, #128): Add guard page to new threads.
                /*
                thread_info::set(imp::guard::current(), their_thread);
                let try_result = panic::catch_unwind(panic::AssertUnwindSafe(f));
                *their_packet.get() = Some(try_result);
                */
                let result = f();
                *their_packet.get() = Some(Ok(result));
            }
        };

        Ok(JoinHandle(JoinInner {
            native: unsafe {
                Some(imp::Thread::new(stack_size, Box::new(main))?)
            },
            thread: my_thread,
            packet: Packet(my_packet),
        }))
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub fn spawn<F, T>(f: F) -> JoinHandle<T> where
    F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
{
    Builder::new().spawn(f).unwrap()
}

#[stable(feature = "rust1", since = "1.0.0")]
pub fn yield_now() {
    imp::Thread::yield_now()
}

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_deprecated(since = "1.6.0", reason = "replaced by `std::thread::sleep`")]
pub fn sleep_ms(ms: u32) {
    sleep(Duration::from_millis(ms as u64))
}

#[stable(feature = "thread_sleep", since = "1.4.0")]
pub fn sleep(dur: Duration) {
    imp::Thread::sleep(dur)
}

#[unstable(feature = "thread_id", issue = "21507")]
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct ThreadId(u64);

impl ThreadId {
    // Generate a new unique thread ID.
    fn new() -> ThreadId {
        // TODO(steed, #129): Use `std`s implementation (commented out below).
        /*
        static GUARD: mutex::Mutex = mutex::Mutex::new();
        static mut COUNTER: u64 = 0;

        unsafe {
            GUARD.lock();

            // If we somehow use up all our bits, panic so that we're not
            // covering up subtle bugs of IDs being reused.
            if COUNTER == ::u64::MAX {
                GUARD.unlock();
                panic!("failed to generate unique thread ID: bitspace exhausted");
            }

            let id = COUNTER;
            COUNTER += 1;

            GUARD.unlock();

            ThreadId(id)
        }
        */
        static COUNT: AtomicUsize = AtomicUsize::new(0);
        ThreadId(COUNT.fetch_add(1, Ordering::Relaxed) as u64)
    }
}

#[unstable(feature = "thread_id", issue = "21507")]
impl fmt::Debug for ThreadId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("ThreadId { .. }")
    }
}

struct Inner {
    name: Option<CString>,      // Guaranteed to be UTF-8
    id: ThreadId,
}

#[derive(Clone)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Thread {
    inner: Arc<Inner>,
}

impl Thread {
    // Used only internally to construct a thread object without spawning
    fn new(name: Option<String>) -> Thread {
        let cname = name.map(|n| {
            CString::new(n).expect("thread name may not contain interior null bytes")
        });
        Thread {
            inner: Arc::new(Inner {
                name: cname,
                id: ThreadId::new(),
            })
        }
    }
    #[unstable(feature = "thread_id", issue = "21507")]
    pub fn id(&self) -> ThreadId {
        self.inner.id
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn name(&self) -> Option<&str> {
        self.cname().map(|s| unsafe { str::from_utf8_unchecked(s.to_bytes()) } )
    }
    fn cname(&self) -> Option<&CStr> {
        self.inner.name.as_ref().map(|s| &**s)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.name(), f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub type Result<T> = ::result::Result<T, Box<Any + Send + 'static>>;

// This packet is used to communicate the return value between the child thread
// and the parent thread. Memory is shared through the `Arc` within and there's
// no need for a mutex here because synchronization happens with `join()` (the
// parent thread never reads this packet until the child has exited).
//
// This packet itself is then stored into a `JoinInner` which in turns is placed
// in `JoinHandle` and `JoinGuard`. Due to the usage of `UnsafeCell` we need to
// manually worry about impls like Send and Sync. The type `T` should
// already always be Send (otherwise the thread could not have been created) and
// this type is inherently Sync because no methods take &self. Regardless,
// however, we add inheriting impls for Send/Sync to this type to ensure it's
// Send/Sync and that future modifications will still appropriately classify it.
struct Packet<T>(Arc<UnsafeCell<Option<Result<T>>>>);

unsafe impl<T: Send> Send for Packet<T> {}
unsafe impl<T: Sync> Sync for Packet<T> {}

struct JoinInner<T> {
    native: Option<imp::Thread>,
    thread: Thread,
    packet: Packet<T>,
}

impl<T> JoinInner<T> {
    fn join(&mut self) -> Result<T> {
        self.native.take().unwrap().join();
        unsafe {
            (*self.packet.0.get()).take().unwrap()
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct JoinHandle<T>(JoinInner<T>);

impl<T> JoinHandle<T> {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn thread(&self) -> &Thread {
        &self.0.thread
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn join(mut self) -> Result<T> {
        self.0.join()
    }
}

impl<T> AsInner<imp::Thread> for JoinHandle<T> {
    fn as_inner(&self) -> &imp::Thread { self.0.native.as_ref().unwrap() }
}

impl<T> IntoInner<imp::Thread> for JoinHandle<T> {
    fn into_inner(self) -> imp::Thread { self.0.native.unwrap() }
}

#[stable(feature = "std_debug", since = "1.16.0")]
impl<T> fmt::Debug for JoinHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("JoinHandle { .. }")
    }
}

fn _assert_sync_and_send() {
    fn _assert_both<T: Send + Sync>() {}
    _assert_both::<JoinHandle<()>>();
    _assert_both::<Thread>();
}

pub fn panicking() -> bool {
    false
}
