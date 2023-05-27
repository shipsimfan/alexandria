use super::QueueFamilyIndices;
use crate::{create_error, Result};
use std::collections::HashSet;

const REQUIRED_EXTENSIONS: &[&str] = &["VK_KHR_swapchain"];

pub(super) fn is_compatible(
    device: &vulkan::PhysicalDevice,
    queue_families: &QueueFamilyIndices,
    surface: &vulkan::Surface,
) -> Result<bool> {
    Ok(is_surface_supported(device, queue_families, surface)? && has_required_extensions(device)?)
}

fn is_surface_supported(
    device: &vulkan::PhysicalDevice,
    queue_families: &QueueFamilyIndices,
    surface: &vulkan::Surface,
) -> Result<bool> {
    Ok(queue_families.has_graphics()
        && device
            .get_surface_support(queue_families.graphics(), surface)
            .map_err(|error| create_error!(EnumerateDevicesFailed, Some(Vulkan(error))))?)
}

fn has_required_extensions(device: &vulkan::PhysicalDevice) -> Result<bool> {
    let extensions = device
        .enumerate_extension_properties(None)
        .map_err(|error| create_error!(EnumerateDevicesFailed, Some(Vulkan(error))))?;

    let mut required_extensions: HashSet<&str> =
        HashSet::from_iter(REQUIRED_EXTENSIONS.iter().map(|str| *str));

    for extension in extensions {
        required_extensions.remove(extension.name.as_str());
    }

    Ok(required_extensions.len() == 0)
}
