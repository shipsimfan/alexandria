//! Subsystem for interacting with and controlling GPUs

use util::*;

mod device;
mod instance;
mod misc;
mod subsystem;
mod util;

pub use device::*;
pub use instance::*;
pub use misc::*;
pub use subsystem::GpuSubsystem;

pub use alexandria_macros::compile_shader;
pub use vulkan::{
    VK_QUEUE_FAMILY_IGNORED, VkAccessFlag2 as VulkanAccessFlag,
    VkAccessFlags2 as VulkanAccessFlags, VkAttachmentLoadOp as VulkanAttachmentLoadOp,
    VkAttachmentStoreOp as VulkanAttachmentStoreOp,
    VkCommandBufferLevel as VulkanCommandBufferLevel,
    VkCommandPoolCreateFlag as VulkanCommandPoolCreateFlag,
    VkCommandPoolCreateFlags as VulkanCommandPoolCreateFlags,
    VkComponentMapping as VulkanComponentMapping, VkComponentSwizzle as VulkanComponentSwizzle,
    VkCullModeFlag as VulkanCullModeFlag, VkCullModeFlags as VulkanCullModeFlags,
    VkDependencyFlag as VulkanDependencyFlag, VkDependencyFlags as VulkanDependencyFlags,
    VkDeviceQueueCreateFlag as VulkanQueueCreateFlag,
    VkDeviceQueueCreateFlags as VulkanQueueCreateFlags, VkFenceCreateFlag as VulkanFenceCreateFlag,
    VkFenceCreateFlags as VulkanFenceCreateFlags, VkFormat as VulkanFormat,
    VkFrontFace as VulkanFrontFace, VkImageAspectFlag as VulkanImageAspectFlag,
    VkImageAspectFlags as VulkanImageAspectFlags, VkImageLayout as VulkanImageLayout,
    VkImageUsageFlag as VulkanImageUsageFlag, VkImageUsageFlags as VulkanImageUsageFlags,
    VkImageViewCreateFlag as VulkanImageViewCreateFlag,
    VkImageViewCreateFlags as VulkanImageViewCreateFlags, VkImageViewType as VulkanImageViewType,
    VkInstanceCreateFlag as VulkanInstanceCreateFlag,
    VkInstanceCreateFlags as VulkanInstanceCreateFlags, VkPhysicalDeviceType as VulkanAdapterKind,
    VkPipelineShaderStageCreateFlag as VulkanPipelineShaderStageCreateFlag,
    VkPipelineShaderStageCreateFlags as VulkanPipelineShaderStageCreateFlags,
    VkPipelineStageFlag2 as VulkanPipelineStageFlag,
    VkPipelineStageFlags2 as VulkanPipelineStageFlags, VkPolygonMode as VulkanPolygonMode,
    VkPrimitiveTopology as VulkanPrimitiveTopology, VkQueueFlag as VulkanQueueFlag,
    VkQueueFlags as VulkanQueueFlags, VkRenderingFlag as VulkanRenderingFlag,
    VkRenderingFlags as VulkanRenderingFlags, VkResolveModeFlag as VulkanResolveModeFlag,
    VkResolveModeFlags as VulkanResolveModeFlags, VkSampleCountFlag as VulkanSampleCountFlag,
    VkSampleCountFlags as VulkanSampleCountFlags, VkShaderStageFlag as VulkanShaderStageFlag,
    VkShaderStageFlags as VulkanShaderStageFlags, VkSharingMode as VulkanSharingMode,
    VkSubmitFlag as VulkanSubmitFlag, VkSubmitFlags as VulkanSubmitFlags,
    VkVertexInputRate as VulkanVertexInputRate,
    ext_debug_utils::{
        VkDebugUtilsMessageSeverityFlagExt as VulkanDebugMessageSeverityFlag,
        VkDebugUtilsMessageSeverityFlagsExt as VulkanDebugMessageSeverityFlags,
        VkDebugUtilsMessageTypeFlagExt as VulkanDebugMessageTypeFlag,
        VkDebugUtilsMessageTypeFlagsExt as VulkanDebugMessageTypeFlags,
    },
    khr_surface::{
        VkColorSpaceKhr as VulkanColorSpace, VkCompositeAlphaFlagKhr as VulkanCompositeAlphaFlag,
        VkCompositeAlphaFlagsKhr as VulkanCompositeAlphaFlags,
        VkPresentModeKhr as VulkanPresentMode, VkSurfaceFormatKhr as VulkanSurfaceFormat,
        VkSurfaceTransformFlagKhr as VulkanSurfaceTransformFlag,
        VkSurfaceTransformFlagsKhr as VulkanSurfaceTransformFlags,
    },
    khr_swapchain::{
        VkSwapchainCreateFlagKhr as VulkanSwapchainCreateFlag,
        VkSwapchainCreateFlagsKhr as VulkanSwapchainCreateFlags,
    },
};
