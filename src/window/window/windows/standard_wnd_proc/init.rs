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
    ) {
        self.rect = current_rect;
        self.windowed_rect = Recti::new(
            requested_position.unwrap_or(current_rect.position),
            requested_size.unwrap_or(current_rect.size),
        );
        self.is_fullscreen = is_fullscreen;
    }
}
