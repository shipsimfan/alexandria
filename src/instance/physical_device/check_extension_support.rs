use vk::{DeviceExtension, PhysicalDevice};

const REQUIRED_EXTENSIONS: &[DeviceExtension] = &[DeviceExtension::Swapchain];

/// Checks if the required extensions are supported
pub(super) fn check_extension_support(
    physical_device: &PhysicalDevice,
) -> Option<Vec<DeviceExtension>> {
    let extensions = REQUIRED_EXTENSIONS.to_vec();
    let supported_extensions = physical_device.device_extensions().ok()?;

    let mut required_extensions = extensions.clone();
    for extension in supported_extensions {
        for i in 0..required_extensions.len() {
            if required_extensions[i] == extension {
                required_extensions.swap_remove(i);
                break;
            }
        }
    }

    if required_extensions.len() > 0 {
        return None;
    }

    Some(extensions)
}
