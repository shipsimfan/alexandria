use crate::{DisplayMode, Result, WindowError};
use alexandria_math::Vector2u;
use win32::{AdjustWindowRectEx, FALSE, RECT, try_get_last_error};

impl DisplayMode {
    /// Converts client area size to window size based on display mode
    pub(crate) fn client_to_window(&self, size: Vector2u) -> Result<Vector2u> {}
}
