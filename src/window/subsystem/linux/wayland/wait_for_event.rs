use crate::{Error, Notify, Result, window::subsystem::linux::WaylandWindowSubsystem};
use linux::{
    poll::{POLLIN, poll, pollfd},
    try_linux,
};
use std::time::Duration;

impl<UserEvent: 'static + Send> WaylandWindowSubsystem<UserEvent> {
    /// Wait for an event to occur on the window subsystem, `notify` signals, or `timeout` passes
    pub(in crate::window::subsystem::linux) fn wait_for_event(
        &self,
        notify: &Notify,
        timeout: Option<Duration>,
    ) -> Result<bool> {
        // Dispatch events before reading
        while !self.connection.prepare_read() {
            self.connection.dispatch_pending()?;
        }

        // Flush pending requests
        if let Err(error) = self.connection.flush() {
            self.connection.cancel_read();
            return Err(error);
        }

        // Wait for an event
        let mut fds: [pollfd; 2] = [
            pollfd {
                fd: self.connection.get_fd(),
                events: POLLIN as _,
                revents: 0,
            },
            pollfd {
                fd: unsafe { notify.inner().get_fd() },
                events: POLLIN as _,
                revents: 0,
            },
        ];

        loop {
            match try_linux!(poll(
                fds.as_mut_ptr(),
                fds.len() as _,
                timeout.map(|t| t.as_millis() as _).unwrap_or(-1)
            )) {
                Ok(_) => break,
                Err(linux::Error::EINTR) => {}
                Err(linux::Error::ETIMEDOUT) => {
                    self.connection.cancel_read();
                    return Ok(false);
                }
                Err(error) => {
                    self.connection.cancel_read();
                    return Err(Error::new_with("unable to wait for an event", error));
                }
            }
        }

        // Check the result of the Wayland socket
        if fds[0].revents & (POLLIN as i16) != 0 {
            self.connection.read_events()?;
        } else {
            self.connection.cancel_read();
        }

        // Check the result of the user wake event
        if fds[1].revents & (POLLIN as i16) != 0 {
            notify.reset()?;
        }

        self.connection.dispatch_pending()?;
        Ok(true)
    }
}
