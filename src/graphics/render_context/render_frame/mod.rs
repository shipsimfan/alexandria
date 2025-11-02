use crate::{graphics::RenderContext, LogCallbacks};

mod drop;
mod end;
mod new;
mod render;
mod set_active;

/// The context for actively rendering a frame
#[must_use]
pub struct RenderFrame<'a> {
    /// The graphics context that actually performs rendering
    render_context: &'a mut RenderContext,

    /// The callbacks for logs
    log_callbacks: &'a mut dyn LogCallbacks,

    /// Should the present be aligned with vertical sync?s
    vsync: bool,

    /// Has the `end` function been called?
    frame_ended: bool,
}
