use crate::{
    Error, Result,
    math::{Recti, Vector2i, Vector2u},
    window::StandardWndProc,
};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Set the minimum size of the window, in pixels
    pub(in crate::window::window::windows) fn set_minimum_size(
        &mut self,
        minimum_size: Option<Vector2u>,
    ) -> Result<()> {
        self.minimum_client_size = minimum_size;
        self.minimum_window_size = match minimum_size {
            Some(minimum_size) => Some(
                self.style
                    .client_to_window(Recti::new(
                        Vector2i::new(0, 0),
                        minimum_size.map(|v| v as _),
                    ))
                    .map_err(|os| Error::new_with("unable to set minimum window size", os))?
                    .size
                    .map(|v| v as _),
            ),
            None => None,
        };
        Ok(())
    }
}
