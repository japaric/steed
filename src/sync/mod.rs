#![stable(feature = "steed", since = "1.0.0")]

#[stable(feature = "steed", since = "1.0.0")]
pub use alloc::arc::{Arc, Weak};
#[stable(feature = "steed", since = "1.0.0")]
pub use core::sync::atomic;
#[stable(feature = "steed", since = "1.0.0")]
pub use self::mutex::{Mutex, MutexGuard};

mod mutex;
