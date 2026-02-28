use crate::{
    Result,
    window::{XdgOutputManager, display::linux::wayland::WaylandDisplay},
};
use std::rc::Rc;

impl<UserEvent: 'static + Send> WaylandDisplay<UserEvent> {
    /// Upgrade this display to an [`XdgOutput`] if possible
    pub(in crate::window::display::linux) fn upgrade(
        &mut self,
        xdg_output_manager: &Rc<XdgOutputManager>,
        emit_events: bool,
    ) -> Result<()> {
        let output = match self {
            WaylandDisplay::Wl(output) => output,
            WaylandDisplay::Xdg(_) => return Ok(()),
        };

        if !emit_events {
            output.data_mut().disable_events();
        }

        xdg_output_manager
            .get_xdg_output(output.clone())
            .map(|xdg_output| *self = WaylandDisplay::Xdg(xdg_output))
            .map_err(|(error, output)| {
                *self = WaylandDisplay::Wl(output);
                error
            })
    }
}
