use crate::{Result, Window, platform::linux::WindowKind};

impl Window {
    /// Block for a message from the window system
    ///
    /// On Windows, this function processes the message that caused the thread to wake
    pub fn wait_for_message(&mut self) -> Result<()> {
        match &mut self.kind {
            WindowKind::Wayland(wayland) => wayland.wait_for_message(),
            WindowKind::X11(_) => todo!(),
        }
    }
}
