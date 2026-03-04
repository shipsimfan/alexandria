use crate::{EventKind, Result, math::Recti, window::StandardWndProc};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Called when the window is resized or moved
    pub(in crate::window::window::windows::standard_wnd_proc) fn change_rect(
        &mut self,
        new_rect: Recti,
    ) -> Result<()> {
        if let Some(id) = self.id {
            if new_rect.size != self.rect.size {
                self.event_queue.push(EventKind::WindowResized {
                    id,
                    new_size: new_rect.size,
                })?;
            }

            if new_rect.position != self.rect.position {
                self.event_queue.push(EventKind::WindowMoved {
                    id,
                    new_position: new_rect.position,
                })?;
            }
        }

        self.rect = new_rect;
        if !self.is_fullscreen {
            self.windowed_rect = new_rect;
        }

        Ok(())
    }
}
