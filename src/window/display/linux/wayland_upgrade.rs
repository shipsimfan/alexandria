use crate::{
    Result,
    window::{XdgOutputManager, display::DisplayInner},
};
use std::rc::Rc;

// rustdoc imports
#[allow(unused_imports)]
use crate::window::display::linux::wayland::{WlOutput, XdgOutput};

impl<UserEvent: 'static + Send> DisplayInner<UserEvent> {
    /// Update this display from a [`WlOutput`] to an [`XdgOutput`], if it is currently a [`WlOutput`].
    pub(in crate::window) fn wayland_upgrade(
        &mut self,
        xdg_output_manager: &Rc<XdgOutputManager>,
        emit_events: bool,
    ) -> Result<()> {
        match self {
            DisplayInner::Wayland(wayland) => wayland.upgrade(xdg_output_manager, emit_events),
            DisplayInner::X11 => Ok(()),
        }
    }
}
