use crate::{
    Id,
    window::{Window, WindowSubsystem},
};

impl<UserEvent: 'static + Send> WindowSubsystem<UserEvent> {
    /// Destroy the window with the given ID, removing it from the system and freeing its resources
    pub fn destroy_window(&self, id: Id<Window<UserEvent>>) {
        self.inner.borrow_mut().destroy_window(unsafe { id.cast() });
    }
}
