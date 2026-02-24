use crate::{
    Error, Result,
    math::{Recti, Vector2i, Vector2u},
    window::StandardWndProc,
};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Set the maximum size of the window, in pixels
    pub(in crate::window::window::windows) fn set_maximum_size(
        &mut self,
        maximum_size: Option<Vector2u>,
    ) -> Result<()> {
        self.maximum_client_size = maximum_size;
        self.maximum_window_size = match maximum_size {
            Some(maximum_size) => Some(
                self.style
                    .client_to_window(Recti::new(
                        Vector2i::new(0, 0),
                        maximum_size.map(|v| v as _),
                    ))
                    .map_err(|os| Error::new_with("unable to set maximum window size", os))?
                    .size
                    .map(|v| v as _),
            ),
            None => None,
        };
        Ok(())
    }
}
