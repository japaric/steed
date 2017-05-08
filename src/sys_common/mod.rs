// Rust 1.16.0 (no tests)
pub mod io;
// Rust 1.16.0
pub mod mutex;
// Rust 1.16.0 (no tests, missing support for `lookup_host`)
pub mod net;
// Rust 1.16.0
pub mod poison;
// Rust 1.15.0
pub mod thread;

pub mod util;

/// A trait for viewing representations from std types
#[doc(hidden)]
pub trait AsInner<Inner: ?Sized> {
    fn as_inner(&self) -> &Inner;
}

/// A trait for viewing representations from std types
#[doc(hidden)]
pub trait AsInnerMut<Inner: ?Sized> {
    fn as_inner_mut(&mut self) -> &mut Inner;
}

/// A trait for extracting representations from std types
#[doc(hidden)]
pub trait IntoInner<Inner> {
    fn into_inner(self) -> Inner;
}

/// A trait for creating std types from internal representations
#[doc(hidden)]
pub trait FromInner<Inner> {
    fn from_inner(inner: Inner) -> Self;
}

/// One-time runtime cleanup.
pub fn cleanup() { }
