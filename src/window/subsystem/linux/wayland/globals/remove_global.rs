use crate::{EventKind, Result, window::subsystem::linux::wayland::WaylandGlobals};

impl<UserEvent: 'static + Send> WaylandGlobals<UserEvent> {
    /// Removes a global from the list of globals
    pub fn remove_global(&mut self, name: u32) -> Result<()> {
        // Check if the global is a display
        for (display_id, display) in self.displays.key_value_iter() {
            if name != display.wayland_name().unwrap() {
                continue;
            }

            self.displays.remove(display_id);
            if self.events_enabled {
                self.event_queue.push(EventKind::DisplayRemoved {
                    id: unsafe { display_id.cast() },
                })?;
            }

            return Ok(());
        }

        // Check if an optional global is removed
        if let Some(xdg_output) = &self.xdg_output_manager {
            if name == xdg_output.name() {
                for display in &mut self.displays {
                    display.wayland_downgrade();
                }
                self.xdg_output_manager = None;
                return Ok(());
            }
        }

        // TODO: Check if a required global is removed

        Ok(())
    }
}
