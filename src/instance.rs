use crate::{create_error, os, Result};
use std::sync::Arc;
use vulkan::{VkApplicationInfo, VkInstance, Vulkan, VK_API_VERSION_1_0};

pub struct Instance {
    vk_instance: VkInstance,
    os_instance: os::Instance,
}

impl Instance {
    pub fn new(app_name: &str, app_version: u32) -> Result<Arc<Self>> {
        assert_eq!(app_name.as_bytes()[app_name.len() - 1], 0);

        let os_instance = os::Instance::new(app_name)
            .map_err(|error| create_error!(OsInstanceCreationFailed, Some(OS(error))))?;

        let vulkan = Vulkan::new_native()
            .map_err(|error| create_error!(VulkanInstanceCreationFailed, Some(Vulkan(error))))?;

        let vk_instance = vulkan
            .create_instance(&vulkan::VkInstanceCreateInfo::new(
                vulkan::VkInstanceCreateFlags::new(&[]),
                Some(&VkApplicationInfo::new(
                    Some(app_name),
                    app_version,
                    Some("Alexandria\0"),
                    0,
                    VK_API_VERSION_1_0,
                )),
                &[],
                os::get_extension_list(),
            ))
            .map_err(|error| create_error!(VulkanInstanceCreationFailed, Some(Vulkan(error))))?;

        Ok(Arc::new(Instance {
            vk_instance,
            os_instance,
        }))
    }

    pub(crate) fn os_instance(&self) -> &os::Instance {
        &self.os_instance
    }
}
