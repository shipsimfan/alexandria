use crate::{
    Id,
    window::{StandardWndProc, Window},
};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Set the ID of the window to push events with
    pub(in crate::window::window::windows) fn set_id(&mut self, id: Id<Window<UserEvent>>) {
        self.id = Some(id);
    }
}
