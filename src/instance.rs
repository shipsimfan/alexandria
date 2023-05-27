use crate::{create_error, os, Result, Version, Window};
use std::sync::Arc;

pub struct Instance {
    vk_instance: Arc<vulkan::Instance>,
    os_instance: os::Instance,
}

#[cfg(debug_assertions)]
const ENABLE_VALIDATION_LAYERS: bool = true;

#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

fn is_validation_supported(library: &vulkan::Library) -> Result<bool> {
    let layers = library
        .enumerate_layer_properties()
        .map_err(|error| create_error!(VulkanInstanceCreationFailed, Some(Vulkan(error))))?;

    for layer in layers {
        if layer.name == "VK_LAYER_KHRONOS_validation" {
            return Ok(true);
        }
    }

    Ok(false)
}

impl Instance {
    pub fn new(app_name: String, app_version: Version) -> Result<Arc<Self>> {
        let os_instance = os::Instance::new(&app_name)
            .map_err(|error| create_error!(OsInstanceCreationFailed, Some(OS(error))))?;

        let library = vulkan::Library::new_native()
            .map_err(|error| create_error!(VulkanInstanceCreationFailed, Some(Vulkan(error))))?;

        let layers = if ENABLE_VALIDATION_LAYERS {
            if is_validation_supported(&library)? {
                vec!["VK_LAYER_KHRONOS_validation".to_owned()]
            } else {
                return Err(create_error!(VulkanInstanceCreationFailed, None));
            }
        } else {
            Vec::new()
        };

        let mut extensions = vec!["VK_KHR_surface".to_owned()];
        extensions.extend(os::get_extension_list());

        let vk_instance = library
            .create_instance(vulkan::InstanceCreateInfo {
                application_info: Some(vulkan::ApplicationInfo {
                    name: Some(app_name),
                    version: Some(app_version),
                    engine_name: Some("Alexandria".to_owned()),
                    engine_version: Some(Version::new(0, 0, 0, 0)),
                    api_version: vulkan::VK_API_VERSION_1_0,
                }),
                enabled_layers: layers,
                enabled_extensions: extensions,
            })
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

    pub(crate) fn vk_instance(&self) -> &Arc<vulkan::Instance> {
        &self.vk_instance
    }

    pub(crate) fn os_instance(&self) -> &os::Instance {
        &self.os_instance
    }
}
