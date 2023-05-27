use crate::{create_error, Result};

pub(crate) struct SwapchainSettings {
    pub(crate) format: vulkan::SurfaceFormat,
    pub(crate) present_mode: vulkan::PresentMode,
    pub(crate) swap_extent: vulkan::Extent2D,
    pub(crate) image_count: u32,
    pub(crate) current_transform: vulkan::SurfaceTransformFlagBits,
}

fn get_swap_surface_format(
    surface: &vulkan::Surface,
    device: &vulkan::PhysicalDevice,
) -> Result<Option<vulkan::SurfaceFormat>> {
    let mut formats = device
        .get_surface_formats(surface)
        .map_err(|error| create_error!(EnumerateDevicesFailed, Some(Vulkan(error))))?;

    for i in 0..formats.len() {
        if formats[i].format == vulkan::Format::R8G8B8A8SRGB
            && formats[i].color_space == vulkan::ColorSpace::SRGBNonLinear
        {
            return Ok(Some(formats.swap_remove(i)));
        }
    }

    Ok(if formats.len() == 0 {
        None
    } else {
        Some(formats.swap_remove(0))
    })
}

fn get_present_mode(
    surface: &vulkan::Surface,
    device: &vulkan::PhysicalDevice,
) -> Result<vulkan::PresentMode> {
    let present_modes = device
        .get_surface_present_modes(surface)
        .map_err(|error| create_error!(EnumerateDevicesFailed, Some(Vulkan(error))))?;

    for present_mode in present_modes {
        if present_mode == vulkan::PresentMode::Mailbox {
            return Ok(present_mode);
        }
    }

    Ok(vulkan::PresentMode::FIFO)
}

fn get_swap_extent(current_extent: vulkan::Extent2D) -> Option<vulkan::Extent2D> {
    if current_extent.width != u32::MAX {
        Some(current_extent)
    } else {
        None
    }
}

fn get_image_count(min_image_count: u32, max_image_count: u32) -> u32 {
    if max_image_count > 0 && min_image_count + 1 > max_image_count {
        max_image_count
    } else {
        min_image_count + 1
    }
}

impl SwapchainSettings {
    pub(super) fn get(
        surface: &vulkan::Surface,
        device: &vulkan::PhysicalDevice,
    ) -> Result<Option<Self>> {
        let capabilities = device
            .get_surface_capabilities(surface)
            .map_err(|error| create_error!(EnumerateDevicesFailed, Some(Vulkan(error))))?;

        let format = match get_swap_surface_format(surface, device)? {
            Some(format) => format,
            None => return Ok(None),
        };
        let present_mode = get_present_mode(surface, device)?;
        let swap_extent = match get_swap_extent(capabilities.current_extent) {
            Some(swap_extent) => swap_extent,
            None => return Ok(None),
        };

        Ok(Some(SwapchainSettings {
            format,
            present_mode,
            swap_extent,
            image_count: get_image_count(
                capabilities.min_image_count,
                capabilities.max_image_count,
            ),
            current_transform: capabilities.current_transform,
        }))
    }
}
