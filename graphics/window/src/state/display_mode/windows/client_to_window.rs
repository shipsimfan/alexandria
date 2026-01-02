use crate::{DisplayMode, Result, WindowError};
use alexandria_math::Vector2u;
use win32::{AdjustWindowRectEx, FALSE, RECT, try_get_last_error};

impl DisplayMode {
    /// Converts client area size to window size based on display mode
    pub(crate) fn client_to_window(&self, size: Vector2u) -> Result<Vector2u> {
        let (style, ex_style) = self.style();

        let mut rect = RECT {
            left: 0,
            top: 0,
            right: (size.x as i32),
            bottom: (size.y as i32),
        };

        try_get_last_error!(AdjustWindowRectEx(&mut rect, style, FALSE, ex_style))
            .map_err(|os| WindowError::new_os("unable to convert window size", os))?;

        Ok(Vector2u::new(
            (rect.right - rect.left) as _,
            (rect.bottom - rect.top) as _,
        ))
    }
}
