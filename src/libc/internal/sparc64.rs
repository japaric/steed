pub unsafe fn set_thread_pointer(thread_data: *mut ()) {
    let _ = thread_data; // TODO(steed, #127): Set thread-local pointer.
}
