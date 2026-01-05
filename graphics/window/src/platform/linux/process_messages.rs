use crate::{Result, Window, platform::linux::WindowKind};

impl Window {
    /// Process all messages that have occurred since the last call
    ///
    /// If none have happened, this function will return immediately
    pub fn process_messages(&mut self) -> Result<()> {
        match &mut self.kind {
            WindowKind::Wayland(wayland) => wayland.process_messages(),
            WindowKind::X11(_) => todo!(),
        }
    }
}
