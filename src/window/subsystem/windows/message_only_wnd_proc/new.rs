use crate::window::subsystem::windows::MessageOnlyWndProc;

impl MessageOnlyWndProc {
    /// Create a new [`MessageOnlyWndProc`]
    pub fn new() -> MessageOnlyWndProc {
        MessageOnlyWndProc {
            enumerate_displays: false,
            refresh_dpi: false,
        }
    }
}
