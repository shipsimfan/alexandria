use crate::window::WlOutput;

impl<T> WlOutput<T> {
    /// Create an owned version that takes the listener data
    pub(in crate::window::display::linux::wayland) fn clone(&mut self) -> Self {
        self.drop = false;
        WlOutput {
            handle: self.handle,
            name: self.name,
            drop: true,
            listener_data: self.listener_data.take(),
            connection: self.connection.clone(),
        }
    }
}
