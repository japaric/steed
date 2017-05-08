use libc::SINGLE_THREADED_TLS;
pub unsafe fn set_thread_pointer(thread_data: *mut ()) {
    // TODO(steed, #127): Set thread-local pointer.
    SINGLE_THREADED_TLS = thread_data as *mut _;
}
