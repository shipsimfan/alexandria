use crate::{
    EventQueue, Result,
    window::subsystem::linux::{
        WaylandWindowSubsystem,
        wayland::{WaylandGlobals, WlDisplay},
    },
};
use std::rc::Rc;

impl<UserEvent: Send> WaylandWindowSubsystem<UserEvent> {
    /// Create a new [`WaylandWindowSubsystem`]
    pub(in crate::window::subsystem::linux) fn new(
        connection: Rc<WlDisplay>,
        event_queue: EventQueue<UserEvent>,
    ) -> Result<WaylandWindowSubsystem<UserEvent>> {
        // Get the registered globals
        let mut registry = connection
            .clone()
            .get_registry()?
            .add_listener(WaylandGlobals::new())?;
        connection.roundtrip()?;

        // Make sure all required global were bound
        if let Err(error) = registry.data_mut().result() {
            return Err(error);
        }

        /*
        if registry.data().compositor().is_none() {
            return Err(WindowError::new("no Wayland compositor available"));
        }
        if registry.data().xdg_wm_base().is_none() {
            return Err(WindowError::new("no XDG window manager available"));
        }
        */

        Ok(WaylandWindowSubsystem {
            connection,
            event_queue,
        })
    }
}
