use crate::{Result, WindowError, WindowEvents, platform::linux::WaylandWindow};
use linux::{
    poll::{POLLIN, poll, pollfd},
    try_linux,
};

impl<Callbacks: WindowEvents> WaylandWindow<Callbacks> {
    /// Process all messages that have occurred since the last call, or block until one arrives
    pub fn process_messages(&mut self) -> Result<()> {
        // Dispatch events before reading
        if !self.display.prepare_read() {
            return self.display.dispatch_pending();
        }

        // Flush pending requests
        if let Err(error) = self.display.flush() {
            self.display.cancel_read();
            return Err(error);
        }

        // Wait for an event
        let mut fds: [pollfd; 2] = [
            pollfd {
                fd: self.display.get_fd(),
                events: POLLIN as _,
                revents: 0,
            },
            pollfd {
                fd: self.wake_handle.get_fd(),
                events: POLLIN as _,
                revents: 0,
            },
        ];
        try_linux!(poll(fds.as_mut_ptr(), fds.len() as _, -1)).map_err(|error| {
            self.display.cancel_read();
            WindowError::new_os("unable to wait for an event", error)
        })?;

        // Check the result of the Wayland socket
        if fds[0].revents & (POLLIN as i16) != 0 {
            self.display.read_events()?;
        } else {
            self.display.cancel_read();
        }

        // Check the result of the user wake event
        if fds[1].revents & (POLLIN as i16) != 0 {
            self.wake_handle.read()?;
        }

        self.display.dispatch_pending()
    }
}
