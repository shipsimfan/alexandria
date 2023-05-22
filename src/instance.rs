use crate::{create_error, os, Result};
use std::sync::Arc;
use vulkan::{VkApplicationInfo, VkInstance, Vulkan, VK_API_VERSION_1_0};

pub struct Instance {
    vk_instance: VkInstance,
    os_instance: os::Instance,
}

const VALIDATION_LAYER_NAME: &str = "VK_LAYER_KHRONOS_validation";
const VALIDATION_LAYER_NAME_TERMINATED: &str = "VK_LAYER_KHRONOS_validation\0";

fn collect_extensions() -> Vec<*const u8> {
    os::get_extension_list()
}

fn collect_layers(vulkan: &Vulkan) -> Result<Vec<*const u8>> {
    #[cfg(debug_assertions)]
    {
        for layer in vulkan
            .enumerate_instance_layer_properties()
            .map_err(|error| create_error!(VulkanInstanceCreationFailed, Some(Vulkan(error))))?
        {
            if layer.layer_name() == VALIDATION_LAYER_NAME {
                return Ok(vec![VALIDATION_LAYER_NAME_TERMINATED.as_ptr()]);
            }
        }
    }

    Ok(Vec::new())
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
                &collect_layers(&vulkan)?,
                &collect_extensions(),
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
