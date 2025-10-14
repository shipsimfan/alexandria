use crate::graphics::RenderContext;

mod drop;
mod end;
mod new;

/// The context for actively rendering a frame
#[must_use]
pub struct RenderFrame<'a> {
    /// The graphics context that actually performs rendering
    render_context: &'a mut RenderContext,

    /// Should the present be aligned with vertical sync?s
    vsync: bool,

    /// Has the `end` function been called?
    frame_ended: bool,
}
