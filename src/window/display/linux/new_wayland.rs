use crate::{
    EventQueue, Result,
    window::{
        WlRegistryRef, XdgOutputManager,
        display::{DisplayInner, linux::WaylandDisplay},
    },
};
use std::rc::Rc;

impl<UserEvent: 'static + Send> DisplayInner<UserEvent> {
    /// Create a new Wayland [`DisplayInner`]
    pub fn new_wayland(
        registry: &mut WlRegistryRef,
        name: u32,
        version: u32,
        event_queue: EventQueue<UserEvent>,
        xdg_output_manager: Option<&Rc<XdgOutputManager>>,
    ) -> Result<DisplayInner<UserEvent>> {
        WaylandDisplay::new(registry, name, version, event_queue, xdg_output_manager)
            .map(DisplayInner::Wayland)
    }
}
