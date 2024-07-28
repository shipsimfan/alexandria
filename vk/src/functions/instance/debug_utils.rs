use crate::functions::get_instance_proc_addr;
use std::{ffi::CStr, ptr::null_mut};
use vulkan::{
    try_vulkan, VkAllocationCallbacks, VkCreateDebugUtilsMessengerEXT,
    VkDebugUtilsMessengerCreateInfoEXT, VkDebugUtilsMessengerEXT, VkDestroyDebugUtilsMessengerEXT,
    VkInstance, VkResult, VK_CREATE_DEBUG_UTILS_MESSENGER_EXT,
    VK_DESTROY_DEBUG_UTILS_MESSENGER_EXT,
};

/// Functions for debug utilities
pub(crate) struct DebugUtilsFunctions {
    create_messenger: VkCreateDebugUtilsMessengerEXT,
    destroy_messenger: VkDestroyDebugUtilsMessengerEXT,
}

impl DebugUtilsFunctions {
    /// Loads the functions for debug utilities
    pub(super) fn new(instance: VkInstance) -> Result<Self, &'static CStr> {
        let create_messenger: VkCreateDebugUtilsMessengerEXT =
            get_instance_proc_addr!(instance, VK_CREATE_DEBUG_UTILS_MESSENGER_EXT)?;
        let destroy_messenger: VkDestroyDebugUtilsMessengerEXT =
            get_instance_proc_addr!(instance, VK_DESTROY_DEBUG_UTILS_MESSENGER_EXT)?;

        Ok(DebugUtilsFunctions {
            create_messenger,
            destroy_messenger,
        })
    }

    /// Create a debug messenger object
    pub(crate) fn create_messenger(
        &self,
        instance: VkInstance,
        create_info: &VkDebugUtilsMessengerCreateInfoEXT,
        allocator: *const VkAllocationCallbacks,
    ) -> Result<VkDebugUtilsMessengerEXT, VkResult> {
        let mut messenger = null_mut();
        try_vulkan!((self.create_messenger)(
            instance,
            create_info,
            allocator,
            &mut messenger
        ))
        .map(|_| messenger)
    }

    /// Destroy a debug messenger object
    pub(crate) fn destroy_messenger(
        &self,
        instance: VkInstance,
        messenger: VkDebugUtilsMessengerEXT,
        allocator: *const VkAllocationCallbacks,
    ) {
        (self.destroy_messenger)(instance, messenger, allocator);
    }
}
