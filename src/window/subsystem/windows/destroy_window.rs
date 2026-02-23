use crate::{
    Id,
    window::{subsystem::WindowSubsystemInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Destroy the window with the given `id`
    pub(in crate::window::subsystem) fn destroy_window(&mut self, id: Id<WindowInner<UserEvent>>) {
        let window = match self.windows.remove(id) {
            Some(window) => window,
            None => return,
        };

        if let Some(display) = window.fullscreen_display() {
            if let Some(display) = self.displays.get_mut(display) {
                display.reset_fullscreen_mode().unwrap();
            }
        }
    }
}
