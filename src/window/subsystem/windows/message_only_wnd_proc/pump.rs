use crate::{
    EventKind, EventQueue, PackedMap, Result,
    window::{
        display::DisplayInner,
        subsystem::windows::{MessageOnlyWndProc, message_only_wnd_proc::re_enumerate_displays},
    },
};
use win32::dxgi::IDXGIFactory;

impl MessageOnlyWndProc {
    /// Pump global events
    pub fn pump_events<UserEvent: 'static + Send>(
        &mut self,
        pump: &EventQueue<UserEvent>,
        displays: &mut PackedMap<DisplayInner<UserEvent>>,
        dxgi_factory: &mut IDXGIFactory,
    ) -> Result<()> {
        if self.enumerate_displays {
            self.enumerate_displays = false;
            self.refresh_dpi = false;

            re_enumerate_displays(pump, displays, dxgi_factory)?;
        }

        if self.refresh_dpi {
            self.refresh_dpi = false;

            for (id, display) in displays.key_value_iter_mut() {
                let old_content_scale = display.content_scale();
                display.refresh_content_scale()?;
                if display.content_scale() != old_content_scale {
                    pump.push(EventKind::DisplayContentScaleChanged {
                        id: unsafe { id.cast() },
                        new_content_scale: display.content_scale(),
                    })?;
                }
            }
        }

        Ok(())
    }
}
