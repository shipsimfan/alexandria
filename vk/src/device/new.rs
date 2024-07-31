use crate::{functions::DeviceFunctions, CreateError, Device, PhysicalDevice};
use std::{ptr::null, rc::Rc};
use vulkan::{VkDeviceCreateInfo, VkDeviceQueueCreateInfo, VkPhysicalDeviceFeatures};

impl Device {
    /// Creates a new [`Device`]
    pub fn new(
        physical_device: &PhysicalDevice,
        queue_families: &[u32],
    ) -> Result<Rc<Self>, CreateError> {
        let queue_priority = 1.0;
        let queue_create_info: Vec<_> = queue_families
            .into_iter()
            .map(|queue_family| VkDeviceQueueCreateInfo {
                queue_family_index: *queue_family,
                queue_count: 1,
                queue_priorities: &queue_priority,
                ..Default::default()
            })
            .collect();

        let device_features = VkPhysicalDeviceFeatures::default();

        let create_info = VkDeviceCreateInfo {
            queue_create_infos: queue_create_info.as_ptr(),
            queue_create_info_count: queue_create_info.len() as _,
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
