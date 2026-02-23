use crate::{
    EventQueue, Result,
    window::{
        WlRegistryRef,
        display::{DisplayInner, linux::WaylandDisplay},
    },
};

impl<UserEvent: 'static + Send> DisplayInner<UserEvent> {
    /// Create a new Wayland [`DisplayInner`]
    pub fn new_wayland(
        registry: &mut WlRegistryRef,
        name: u32,
        version: u32,
        event_queue: EventQueue<UserEvent>,
    ) -> Result<DisplayInner<UserEvent>> {
        WaylandDisplay::new(registry, name, version, event_queue).map(DisplayInner::Wayland)
    }
}
