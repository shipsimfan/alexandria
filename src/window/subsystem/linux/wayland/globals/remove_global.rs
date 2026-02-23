use crate::{EventKind, Result, window::subsystem::linux::wayland::WaylandGlobals};

impl<UserEvent: 'static + Send> WaylandGlobals<UserEvent> {
    pub fn remove_global(&mut self, name: u32) -> Result<()> {
        // Check if the global is a display
        for i in 0..self.name_to_display_map.len() {
            if self.name_to_display_map[i].0 == name {
                let id = self.name_to_display_map[i].1;
                self.name_to_display_map.swap_remove(i);

                self.displays.remove(id);
                if self.events_enabled {
                    self.event_queue.push(EventKind::DisplayRemoved {
                        id: unsafe { id.cast() },
                    })?;
                }

                return Ok(());
            }
        }

        // TODO: Check if a required global is removed

        Ok(())
    }
}
