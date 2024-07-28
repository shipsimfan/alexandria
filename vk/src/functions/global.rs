use super::get_instance_proc_addr;
use std::{
    ffi::{c_char, CStr},
    ptr::null_mut,
};
use vulkan::{
    try_vulkan, VkAllocationCallbacks, VkCreateInstance, VkEnumerateInstanceExtensionProperties,
    VkEnumerateInstanceLayerProperties, VkExtensionProperties, VkInstance, VkInstanceCreateInfo,
    VkLayerProperties, VkResult, VK_CREATE_INSTANCE, VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES,
    VK_ENUMERATE_INSTANCE_LAYER_PROPERTIES,
};

/// Global Vulkan functions
pub(crate) struct GlobalFunctions {
    enumerate_instance_layer_properties: VkEnumerateInstanceLayerProperties,
    enumerate_instance_extension_properties: VkEnumerateInstanceExtensionProperties,
    create_instance: VkCreateInstance,
}

impl GlobalFunctions {
    /// Loads the global functions
    pub(crate) fn new() -> Result<Self, &'static CStr> {
        let enumerate_instance_layer_properties =
            get_instance_proc_addr!(VK_ENUMERATE_INSTANCE_LAYER_PROPERTIES)?;
        let enumerate_instance_extension_properties =
            get_instance_proc_addr!(VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES)?;
        let create_instance = get_instance_proc_addr!(VK_CREATE_INSTANCE)?;

        Ok(GlobalFunctions {
            enumerate_instance_extension_properties,
            enumerate_instance_layer_properties,
            create_instance,
        })
    }

    /// Returns up to requested number of global layer properties
    pub(crate) fn enumerate_instance_layer_properties(
        &self,
        property_count: &mut u32,
        properties: *mut VkLayerProperties,
    ) -> Result<(), VkResult> {
        try_vulkan!((self.enumerate_instance_layer_properties)(
            property_count,
            properties
        ))
        .map(|_| ())
    }

    /// Returns up to requested number of global extension properties
    pub(crate) fn enumerate_instance_extension_properties(
        &self,
        layer_name: *const c_char,
        property_count: &mut u32,
        properties: *mut VkExtensionProperties,
    ) -> Result<(), VkResult> {
        try_vulkan!((self.enumerate_instance_extension_properties)(
            layer_name,
            property_count,
            properties
        ))
        .map(|_| ())
    }

    /// Create a new Vulkan instance
    pub(crate) fn create_instance(
        &self,
        create_info: &VkInstanceCreateInfo,
        allocator: *const VkAllocationCallbacks,
    ) -> Result<VkInstance, VkResult> {
        let mut instance = null_mut();
        try_vulkan!((self.create_instance)(
            create_info,
            allocator,
            &mut instance
        ))
        .map(|_| instance)
    }
}
