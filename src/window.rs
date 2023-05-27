use crate::{create_error, os, Device, GraphicsContext, Instance, Result};
use std::sync::Arc;

pub struct Window {
    inner: Box<os::Window>,
    surface: vulkan::Surface,

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
            .create_surface(instance.vk_instance())
            .map_err(|error| create_error!(SurfaceCreationFailed, Some(Vulkan(error))))?;

        Ok(Window {
            inner,
            surface,

            instance,

            ref_count: Arc::new(()),
        })
    }

    // Returns whether or not the window is still alive
    pub fn poll_events(&mut self) -> Result<bool> {
        self.inner
            .poll_events()
            .map_err(|error| create_error!(PollEventsFailed, Some(OS(error))))
    }

    pub fn enumerate_devices(&self) -> Result<Vec<Device>> {
        let devices = self
            .instance
            .vk_instance()
            .enumerate_physical_devices()
            .map_err(|error| create_error!(EnumerateDevicesFailed, Some(Vulkan(error))))?;

        let mut result = devices
            .into_iter()
            .filter_map(|device| Device::new(device, self.instance.clone(), &self.surface))
            .collect::<Result<Vec<Device>>>()?;
        result.sort_by(|device1, device2| device1.score().cmp(&device2.score()));
        Ok(result)
    }

    pub fn create_graphics_context(&mut self, device: Device) -> Result<GraphicsContext> {
        GraphicsContext::new(self, self.ref_count.clone(), device)
    }

    pub(crate) fn surface(&mut self) -> &mut vulkan::Surface {
        &mut self.surface
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        assert_eq!(Arc::strong_count(&self.ref_count), 1);
    }
}
