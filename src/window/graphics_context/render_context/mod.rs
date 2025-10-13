use crate::GraphicsContext;

mod drop;
mod end;
mod new;

/// The context for actively rendering a frame
#[must_use]
pub struct RenderContext<'a> {
    /// The graphics context that actually performs rendering
    graphics_context: &'a mut GraphicsContext,

    /// Should the present be aligned with vertical sync?s
    vsync: bool,

    /// Has the `end` function been called?
    frame_ended: bool,
}
