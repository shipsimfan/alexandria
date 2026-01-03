use crate::{DisplayMode, Result, Window};
use alexandria_math::Vector2u;
use std::borrow::Cow;

impl Window {
    /// Create a new [`Window`]
    pub(crate) fn new(
        title: Cow<'static, str>,
        size: Option<Vector2u>,
        display_mode: DisplayMode,
    ) -> Result<Box<Window>> {
        todo!()
    }
}
