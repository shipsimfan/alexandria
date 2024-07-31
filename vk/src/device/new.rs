use crate::{functions::DeviceFunctions, CreateError, Device, PhysicalDevice};
use std::{ptr::null, rc::Rc};
use vulkan::{VkDeviceCreateInfo, VkDeviceQueueCreateInfo, VkPhysicalDeviceFeatures};

impl Device {
    /// Creates a new [`Device`]
    pub fn new(
        physical_device: &PhysicalDevice,
        queues: Vec<VkDeviceQueueCreateInfo>,
    ) -> Result<Rc<Self>, CreateError> {
        let device_features = VkPhysicalDeviceFeatures::default();

        let create_info = VkDeviceCreateInfo {
            queue_create_infos: queues.as_ptr(),
            queue_create_info_count: queues.len() as _,
            enabled_features: &device_features,
            ..Default::default()
        };

        let handle =
            physical_device
                .f()
                .create_device(physical_device.handle(), &create_info, null())?;
        let instance = physical_device.instance().clone();
        let functions = DeviceFunctions::new(instance.f(), handle)?;

        Ok(Rc::new(Device {
            handle,
            functions,
            instance,
        }))
    }
}
