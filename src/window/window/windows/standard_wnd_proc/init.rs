use crate::{
    math::{Recti, Vector2i},
    window::StandardWndProc,
};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Set-up the initial state of the window post-creation
    pub(in crate::window::window::windows) fn init(
        &mut self,
        current_rect: Recti,
        requested_position: Option<Vector2i>,
        requested_size: Option<Vector2i>,
        is_fullscreen: bool,
        is_maximized: bool,
        is_minimized: bool,
        is_hidden: bool,
        is_bordered: bool,
        is_resizable: bool,
        content_scale: f32,
    ) {
        self.rect = current_rect;
        self.windowed_rect = if is_fullscreen {
            Recti::new(
                requested_position.unwrap_or(current_rect.position),
                requested_size.unwrap_or(current_rect.size),
            )
        } else {
            current_rect
        };
        self.is_fullscreen = is_fullscreen;
        self.is_maximized = is_maximized;
        self.is_minimized = is_minimized;
        self.is_focused = !is_minimized;
        self.is_visible = !is_hidden;
        self.is_borderless = !is_bordered;
        self.is_resizable = is_resizable;
        self.content_scale = content_scale;
    }
}
