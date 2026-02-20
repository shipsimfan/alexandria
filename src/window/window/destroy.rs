use crate::window::Window;

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Destroy the window, removing it from the system and freeing its resources
    pub fn destroy(self) {
        self.context.destroy_window(unsafe { self.id.cast() });
    }
}
