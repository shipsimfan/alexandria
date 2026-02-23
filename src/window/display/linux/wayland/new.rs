use crate::{
    EventQueue, Result,
    window::{
        WlRegistryRef,
        display::linux::{
            WaylandDisplay,
            wayland::{WaylandDisplayEventHandler, WlOutput},
        },
    },
};

impl<UserEvent: 'static + Send> WaylandDisplay<UserEvent> {
    /// Create a new  [`WaylandDisplay`]
    pub fn new(
        registry: &mut WlRegistryRef,
        name: u32,
        version: u32,
        event_queue: EventQueue<UserEvent>,
    ) -> Result<WaylandDisplay<UserEvent>> {
        let output: WlOutput = registry.bind(name, version)?;
        let output = output.add_listener(WaylandDisplayEventHandler::new(event_queue))?;

        Ok(WaylandDisplay { output })
    }
}
