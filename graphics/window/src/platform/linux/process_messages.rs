use crate::{Result, Window, WindowEvents};

impl<Callbacks: WindowEvents> Window<Callbacks> {
    /// Process all messages that have occurred since the last call, or block until one arrives
    pub fn process_messages(&mut self) -> Result<()> {
        match self {
            Window::Wayland(wayland) => wayland.process_messages(),
            Window::X11(_) => todo!(),
        }
    }
}
