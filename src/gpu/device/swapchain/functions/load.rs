use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanSwapchainFunctions, load_device_function},
};
use vulkan::{
    VkDevice,
    khr_swapchain::{
        VK_ACQUIRE_NEXT_IMAGE2_KHR, VK_CREATE_SWAPCHAIN_KHR, VK_DESTROY_SWAPCHAIN_KHR,
        VK_GET_SWAPCHAIN_IMAGES_KHR, VK_QUEUE_PRESENT_KHR,
    },
};

impl VulkanSwapchainFunctions {
    /// Load all the required swapchain functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanSwapchainFunctions> {
        Ok(VulkanSwapchainFunctions {
            create_swapchain: load_device_function!(instance, device, VK_CREATE_SWAPCHAIN_KHR)?,
            destroy_swapchain: load_device_function!(instance, device, VK_DESTROY_SWAPCHAIN_KHR)?,
            get_swapchain_images: load_device_function!(
                instance,
                device,
                VK_GET_SWAPCHAIN_IMAGES_KHR
            )?,
            acquire_next_image2: load_device_function!(
                instance,
                device,
                VK_ACQUIRE_NEXT_IMAGE2_KHR
            )?,
            queue_present: load_device_function!(instance, device, VK_QUEUE_PRESENT_KHR)?,
        })
    }
}
