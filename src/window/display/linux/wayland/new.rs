use crate::{
    EventQueue, Result,
    window::{
        WlRegistryRef, XdgOutputManager,
        display::linux::{
            WaylandDisplay,
            wayland::{WaylandDisplayEventHandler, WlOutput},
        },
    },
};
use std::rc::Rc;

impl<UserEvent: 'static + Send> WaylandDisplay<UserEvent> {
    /// Create a new  [`WaylandDisplay`]
    pub(in crate::window::display::linux) fn new(
        registry: &mut WlRegistryRef,
        name: u32,
        version: u32,
        event_queue: EventQueue<UserEvent>,
        xdg_output_manager: Option<&Rc<XdgOutputManager>>,
    ) -> Result<WaylandDisplay<UserEvent>> {
        let output: WlOutput = registry.bind(name, version)?;
        let output = output.add_listener(WaylandDisplayEventHandler::new(event_queue))?;
        let mut display = WaylandDisplay::Wl(output);

        if let Some(xdg_output_manager) = xdg_output_manager {
            display.upgrade(xdg_output_manager, false)?;
        }

        Ok(display)
    }
}
