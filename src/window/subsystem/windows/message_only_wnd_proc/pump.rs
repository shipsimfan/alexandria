use crate::{
    EventQueue, PackedMap, Result,
    window::{
        display::DisplayInner,
        subsystem::windows::{MessageOnlyWndProc, message_only_wnd_proc::re_enumerate_displays},
    },
};
use win32::dxgi::IDXGIFactory;

impl MessageOnlyWndProc {
    /// Pump global events
    pub fn pump_events<UserEvent: Send>(
        &mut self,
        pump: &EventQueue<UserEvent>,
        displays: &mut PackedMap<DisplayInner>,
        dxgi_factory: &mut IDXGIFactory,
    ) -> Result<()> {
        if self.enumerate_displays {
            self.enumerate_displays = false;
            self.refresh_dpi = false;

            re_enumerate_displays(pump, displays, dxgi_factory)?;
        }

        if self.refresh_dpi {
            self.refresh_dpi = false;

            println!("Refresh DPI");
        }

        Ok(())
    }
}
