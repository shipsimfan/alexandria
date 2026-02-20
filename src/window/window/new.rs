use crate::{
    Id,
    window::{Window, WindowSubsystem},
};

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Create a new [`Window`]
    pub(in crate::window) fn new(
        id: Id<Window<UserEvent>>,
        context: WindowSubsystem<UserEvent>,
    ) -> Window<UserEvent> {
        Window { id, context }
    }
}
