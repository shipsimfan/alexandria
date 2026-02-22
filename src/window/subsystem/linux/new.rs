use crate::{
    EventQueue, Result,
    window::subsystem::{
        WindowSubsystemInner,
        linux::{WaylandWindowSubsystem, WlDisplay},
    },
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Create a new [`WindowSubsystemInner`]
    pub(in crate::window::subsystem) fn new(
        event_queue: EventQueue<UserEvent>,
    ) -> Result<WindowSubsystemInner<UserEvent>> {
        if let Ok(connection) = WlDisplay::try_connect() {
            return WaylandWindowSubsystem::new(connection, event_queue)
                .map(WindowSubsystemInner::Wayland);
        }

        todo!("try to connect to X11");
    }
}
