pub struct Handler(());

impl Handler {
    pub unsafe fn new() -> Handler {
        // TODO(steed, #132): Implement a stack overflow handler.
        Handler(())
    }
}
