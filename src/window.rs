use crate::{create_error, os, Device, GraphicsContext, Result};
use std::sync::Arc;

pub struct Window {
    graphics_context: Arc<GraphicsContext>,

    inner: Box<os::Window>,
}

impl Window {
    pub fn new(device: Device, title: &str, width: usize, height: usize) -> Result<Self> {
        let inner = os::Window::new(device.instance().os_instance(), title, width, height)
            .map_err(|error| create_error!(WindowCreationFailed, Some(OS(error))))?;

        let graphics_context = GraphicsContext::new(device)?;

        Ok(Window {
            graphics_context,

            inner,
        })
    }

    // Returns whether or not the window is still alive
    pub fn poll_events(&self) -> Result<bool> {
        self.inner
            .poll_events()
            .map_err(|error| create_error!(PollEventsFailed, Some(OS(error))))
    }

    pub fn graphics_context(&self) -> &Arc<GraphicsContext> {
        &self.graphics_context
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        assert_eq!(Arc::strong_count(&self.graphics_context), 1)
    }
}
