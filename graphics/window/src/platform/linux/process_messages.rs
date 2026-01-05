use crate::{Result, Window, platform::linux::WindowKind};

impl Window {
    /// Process all messages that have occurred since the last call, or block until one arrives
    pub fn process_messages(&mut self) -> Result<()> {
        match &mut self.kind {
            WindowKind::Wayland(wayland) => wayland.process_messages(),
            WindowKind::X11(_) => todo!(),
        }
    }
}
