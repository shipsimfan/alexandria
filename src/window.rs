use crate::{create_error, os, Instance, Result};
use std::sync::Arc;

pub struct Window {
    _instance: Arc<Instance>,

    inner: Box<os::Window>,
}

impl Window {
    pub fn new(instance: Arc<Instance>, title: &str, width: usize, height: usize) -> Result<Self> {
        let inner = os::Window::new(instance.os_instance(), title, width, height)
            .map_err(|error| create_error!(WindowCreationFailed, Some(OS(error))))?;

        Ok(Window {
            _instance: instance,

            inner,
        })
    }

    // Returns whether or not the window is still alive
    pub fn poll_events(&self) -> Result<bool> {
        self.inner
            .poll_events()
            .map_err(|error| create_error!(PollEventsFailed, Some(OS(error))))
    }
}
