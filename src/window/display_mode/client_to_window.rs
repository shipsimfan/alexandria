use crate::{
    math::{Vector2i, Vector2u},
    DisplayMode, Result,
};
use win32::{try_get_last_error, AdjustWindowRectEx, FALSE, RECT};

impl DisplayMode {
    /// Converts client area position and size to window position and size based on display mode
    pub(in crate::window) fn client_to_window(
        &self,
        size: Vector2u,
        position: Vector2i,
    ) -> Result<(Vector2u, Vector2i)> {
        let (style, ex_style) = self.style();

        let mut rect = RECT {
            left: position.x,
            top: position.y,
            right: (size.x as i32) + position.x,
            bottom: (size.y as i32) + position.y,
        };

        try_get_last_error!(AdjustWindowRectEx(&mut rect, style, FALSE, ex_style))?;

        Ok((
            Vector2u::new((rect.right - rect.left) as _, (rect.bottom - rect.top) as _),
            Vector2i::new(rect.left, rect.top),
        ))
    }
}
