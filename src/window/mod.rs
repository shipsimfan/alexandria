use crate::{Instance, WindowCreationError};

/// A window which can be rendered to
pub struct Window(Box<vk::Window>);

impl Window {
    /// Creates a new [`Window`]
    pub fn new(
        instance: &Instance,
        title: &str,
        width: usize,
        height: usize,
    ) -> Result<Self, WindowCreationError> {
        vk::Window::new(instance.inner(), title, width, height).map(|window| Window(window))
    }

    /// Polls events for the window, returning if the window should continue to run
    pub fn poll_events(&mut self) -> bool {
        self.0.poll_events()
    }

    /// Signals the window to exit at the next event loop
    pub fn exit(&mut self) {
        self.0.exit()
    }

    /// Gets the underlying Vulkan window
    pub(crate) fn inner(&self) -> &vk::Window {
        &self.0
    }
}
