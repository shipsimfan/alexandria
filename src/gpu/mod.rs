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
    VkAttachmentStoreOp as VulkanAttachmentStoreOp, VkBlendFactor as VulkanBlendFactor,
    VkBlendOp as VulkanBlendOp, VkBufferCreateFlag as VulkanBufferCreateFlag,
    VkBufferCreateFlags as VulkanBufferCreateFlags, VkBufferUsageFlag as VulkanBufferUsageFlag,
    VkBufferUsageFlags as VulkanBufferUsageFlags, VkColorComponentFlag as VulkanColorComponentFlag,
    VkColorComponentFlags as VulkanColorComponentFlags,
    VkCommandBufferLevel as VulkanCommandBufferLevel,
    VkCommandPoolCreateFlag as VulkanCommandPoolCreateFlag,
    VkCommandPoolCreateFlags as VulkanCommandPoolCreateFlags, VkCompareOp as VulkanCompareOp,
    VkComponentMapping as VulkanComponentMapping, VkComponentSwizzle as VulkanComponentSwizzle,
    VkCullModeFlag as VulkanCullModeFlag, VkCullModeFlags as VulkanCullModeFlags,
    VkDependencyFlag as VulkanDependencyFlag, VkDependencyFlags as VulkanDependencyFlags,
    VkDeviceQueueCreateFlag as VulkanQueueCreateFlag,
    VkDeviceQueueCreateFlags as VulkanQueueCreateFlags, VkDynamicState as VulkanDynamicState,
    VkFenceCreateFlag as VulkanFenceCreateFlag, VkFenceCreateFlags as VulkanFenceCreateFlags,
    VkFormat as VulkanFormat, VkFrontFace as VulkanFrontFace,
    VkImageAspectFlag as VulkanImageAspectFlag, VkImageAspectFlags as VulkanImageAspectFlags,
    VkImageLayout as VulkanImageLayout, VkImageUsageFlag as VulkanImageUsageFlag,
    VkImageUsageFlags as VulkanImageUsageFlags, VkImageViewCreateFlag as VulkanImageViewCreateFlag,
    VkImageViewCreateFlags as VulkanImageViewCreateFlags, VkImageViewType as VulkanImageViewType,
    VkInstanceCreateFlag as VulkanInstanceCreateFlag,
    VkInstanceCreateFlags as VulkanInstanceCreateFlags, VkLogicOp as VulkanLogicOp,
    VkMemoryHeapFlag as VulkanMemoryHeapFlag, VkMemoryHeapFlags as VulkanMemoryHeapFlags,
    VkMemoryMapFlag as VulkanMemoryMapFlag, VkMemoryMapFlags as VulkanMemoryMapFlags,
    VkMemoryPropertyFlag as VulkanMemoryPropertyFlag,
    VkMemoryPropertyFlags as VulkanMemoryPropertyFlags, VkPhysicalDeviceType as VulkanAdapterType,
    VkPipelineBindPoint as VulkanPipelineBindPoint,
    VkPipelineColorBlendStateCreateFlag as VulkanPipelineColorBlendStateCreateFlag,
    VkPipelineColorBlendStateCreateFlags as VulkanPipelineColorBlendStateCreateFlags,
    VkPipelineCreateFlag as VulkanPipelineCreateFlag,
    VkPipelineCreateFlags as VulkanPipelineCreateFlags,
    VkPipelineDepthStencilStateCreateFlag as VulkanPipelineDepthStencilStateCreateFlag,
    VkPipelineDepthStencilStateCreateFlags as VulkanPipelineDepthStencilStateCreateFlags,
    VkPipelineLayoutCreateFlag as VulkanPipelineLayoutCreateFlag,
    VkPipelineLayoutCreateFlags as VulkanPipelineLayoutCreateFlags,
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
    VkStencilOp as VulkanStencilOp, VkSubmitFlag as VulkanSubmitFlag,
    VkSubmitFlags as VulkanSubmitFlags, VkVertexInputRate as VulkanVertexInputRate,
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
