use crate::{create_error, os, Device, GraphicsContext, Instance, Result};
use std::sync::Arc;
use vulkan::VkSurfaceKHR;

pub struct Window {
    inner: Box<os::Window>,
    surface: VkSurfaceKHR,

    instance: Arc<Instance>,

    ref_count: Arc<()>,
}

impl Window {
    pub(crate) fn new(
        instance: Arc<Instance>,
        title: &str,
        width: usize,
        height: usize,
    ) -> Result<Self> {
        let inner = os::Window::new(instance.os_instance(), title, width, height)
            .map_err(|error| create_error!(WindowCreationFailed, Some(OS(error))))?;

        let surface = inner
            .create_surface(instance.vulkan_instance())
            .map_err(|error| create_error!(WindowCreationFailed, Some(Vulkan(error))))?;

        Ok(Window {
            inner,
            surface,

            instance,

            ref_count: Arc::new(()),
        })
    }

    pub fn enumerate_devices(&self) -> Result<Vec<Device>> {
        self.instance
            .vulkan_instance()
            .enumerate_physical_devices()
            .map(|devices| {
                devices
                    .into_iter()
                    .filter_map(|device| Device::new(device, &self.surface, self.instance.clone()))
                    .collect()
            })
            .map_err(|error| create_error!(EnumerateDevicesFailed, Some(Vulkan(error))))
    }

    pub fn create_graphics_context(&self, device: Device) -> Result<GraphicsContext> {
        GraphicsContext::new(self.ref_count.clone(), device)
    }

    // Returns whether or not the window is still alive
    pub fn poll_events(&self) -> Result<bool> {
        self.inner
            .poll_events()
            .map_err(|error| create_error!(PollEventsFailed, Some(OS(error))))
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        assert_eq!(Arc::strong_count(&self.ref_count), 1);
    }
}
