use crate::{create_error, os, Result, Window};
use std::{
    ffi::{c_char, CStr},
    sync::Arc,
};
use vulkan::{VkApplicationInfo, VkInstance, Vulkan, VK_API_VERSION_1_0};

pub struct Instance {
    vk_instance: Arc<VkInstance>,
    os_instance: os::Instance,
}

const VALIDATION_LAYER_NAME: &CStr =
    unsafe { std::mem::transmute("VK_LAYER_KHRONOS_validation\0") };
const VALIDATION_LAYER_NAME_TERMINATED: &CStr =
    unsafe { std::mem::transmute("VK_LAYER_KHRONOS_validation\0") };

fn collect_extensions() -> Vec<*const c_char> {
    os::get_extension_list()
}

fn collect_layers(vulkan: &Vulkan) -> Result<Vec<*const c_char>> {
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
    pub fn new(app_name: &CStr, app_version: u32) -> Result<Arc<Self>> {
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
                    Some(CStr::from_bytes_with_nul(b"Alexandria\0").unwrap()),
                    0,
                    VK_API_VERSION_1_0,
                )),
                collect_layers(&vulkan)?.as_slice(),
                collect_extensions().as_slice(),
            ))
            .map_err(|error| create_error!(VulkanInstanceCreationFailed, Some(Vulkan(error))))?;

        Ok(Arc::new(Instance {
            vk_instance,
            os_instance,
        }))
    }

    pub fn create_window(
        self: Arc<Self>,
        title: &str,
        width: usize,
        height: usize,
    ) -> Result<Window> {
        Window::new(self, title, width, height)
    }

    pub(crate) fn vulkan_instance(&self) -> &Arc<VkInstance> {
        &self.vk_instance
    }

    pub(crate) fn os_instance(&self) -> &os::Instance {
        &self.os_instance
    }
}
