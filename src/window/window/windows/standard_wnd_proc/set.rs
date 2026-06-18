use crate::{
    Error, EventKind, Result,
    math::{Recti, Vector2i, Vector2u},
    window::{StandardWndProc, WindowStyle},
};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Set the position of this window when it is windowed
    pub(in crate::window::window::windows) fn set_windowed_position(&mut self, position: Vector2i) {
        self.windowed_rect.position = position;
    }

    /// Set the size of this window when it is windowed
    pub(in crate::window::window::windows) fn set_windowed_size(&mut self, size: Vector2u) {
        self.windowed_rect.size = size;
    }

    /// Set the maximum size of the window, in pixels
    pub(in crate::window::window::windows) fn set_maximum_size(
        &mut self,
        maximum_size: Option<Vector2u>,
    ) -> Result<()> {
        self.maximum_client_size = maximum_size;
        self.maximum_window_size = match maximum_size {
            Some(maximum_size) => Some(
                self.style()
                    .client_to_window(Recti::new(Vector2i::ZERO, maximum_size))
                    .map_err(|os| Error::new_with("unable to set maximum window size", os))?
                    .size,
            ),
            None => None,
        };
        Ok(())
    }

    /// Set the minimum size of the window, in pixels
    pub(in crate::window::window::windows) fn set_minimum_size(
        &mut self,
        minimum_size: Option<Vector2u>,
    ) -> Result<()> {
        self.minimum_client_size = minimum_size;
        self.minimum_window_size = match minimum_size {
            Some(minimum_size) => Some(
                self.style()
                    .client_to_window(Recti::new(Vector2i::ZERO, minimum_size))
                    .map_err(|os| Error::new_with("unable to set minimum window size", os))?
                    .size,
            ),
            None => None,
        };
        Ok(())
    }

    /// Set that this window will be maximized when it is windowed
    pub(in crate::window::window::windows) fn set_maximized(&mut self) {
        if !self.is_fullscreen {
            self.is_maximized_when_windowed = true;
        }
    }

    /// Set that this window will be minimized when it is windowed
    pub(in crate::window::window::windows) fn set_minimized(&mut self) {
        if !self.is_fullscreen {
            self.is_minimized_when_windowed = true;
        }
    }

    /// Set this window to be borderless, returning the new style if it needs to be updated
    pub(in crate::window::window::windows) fn set_borderless(
        &mut self,
        borderless: bool,
    ) -> Option<WindowStyle> {
        self.is_borderless = borderless;
        if self.is_fullscreen {
            None
        } else {
            Some(self.style())
        }
    }

    /// Set this window to be resizable, returning the new style if it needs to be updated
    pub(in crate::window::window::windows) fn set_resizable(
        &mut self,
        resizable: bool,
    ) -> Option<WindowStyle> {
        self.is_resizable = resizable;
        if self.is_fullscreen {
            None
        } else {
            Some(self.style())
        }
    }

    /// Set this window to be fullscreen
    pub(in crate::window::window::windows) fn set_fullscreen(
        &mut self,
        fullscreen: bool,
    ) -> Result<()> {
        self.is_fullscreen = fullscreen;

        if let Some(id) = self.id {
            self.event_queue.push(if fullscreen {
                EventKind::WindowEnteredFullscreen { id }
            } else {
                EventKind::WindowLeftFullscreen { id }
            })?;
        }
        Ok(())
    }
}
