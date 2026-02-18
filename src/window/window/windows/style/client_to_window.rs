use crate::{math::Recti, window::WindowStyle};
use win32::{AdjustWindowRectEx, FALSE, try_get_last_error};

impl WindowStyle {
    /// Converts a client size and position to the window size and position based on the window
    /// style
    pub fn client_to_window(&self, client: Recti) -> win32::Result<Recti> {
        let mut win32_client = client.into();

        try_get_last_error!(AdjustWindowRectEx(
            &mut win32_client,
            self.style,
            FALSE,
            self.ex_style
        ))?;

        Ok(win32_client.into())
    }
}
