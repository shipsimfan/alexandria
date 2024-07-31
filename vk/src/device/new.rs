use crate::{
    functions::DeviceFunctions, CreateError, Device, PhysicalDevice, Queue, QueueCreateInfo,
};
use std::{ptr::null, rc::Rc};
use vulkan::{VkDeviceCreateInfo, VkPhysicalDeviceFeatures};

impl Device {
    /// Creates a new [`Device`]
    pub fn new(
        physical_device: &PhysicalDevice,
        queues: Vec<QueueCreateInfo>,
    ) -> Result<Rc<Self>, CreateError> {
        let queue_create_info: Vec<_> = queues.iter().map(|queue| queue.to_vk()).collect();

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

        let mut queues = Vec::new();
        for queue_family_index in 0..queue_create_info.len() {
            for queue_index in 0..queue_create_info[queue_family_index].queue_count {
                queues.push(Queue::new(functions.get_device_queue(
                    handle,
                    queue_family_index as _,
                    queue_index,
                )));
            }
        }

        Ok(Rc::new(Device {
            queues,
            handle,
            functions,
            instance,
        }))
    }
}
