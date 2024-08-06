use crate::functions::get_instance_proc_addr;
use std::{
    ffi::{c_char, CStr},
    ptr::null_mut,
};
use vulkan::{
    try_vulkan, VkAllocationCallbacks, VkCreateDevice, VkDevice, VkDeviceCreateInfo,
    VkEnumerateDeviceExtensionProperties, VkExtensionProperties, VkGetPhysicalDeviceFeatures,
    VkGetPhysicalDeviceProperties, VkGetPhysicalDeviceQueueFamilyProperties, VkInstance,
    VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties,
    VkQueueFamilyProperties, VkResult, VK_CREATE_DEVICE, VK_ENUMERATE_DEVICE_EXTENSION_PROPERTIES,
    VK_GET_PHYSICAL_DEVICE_FEATURES, VK_GET_PHYSICAL_DEVICE_PROPERTIES,
    VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES,
};

/// Functions for physical devices
pub(crate) struct PhysicalDeviceFunctions {
    get_physical_device_properties: VkGetPhysicalDeviceProperties,
    get_physical_device_features: VkGetPhysicalDeviceFeatures,
    get_physical_device_queue_family_properties: VkGetPhysicalDeviceQueueFamilyProperties,
    create_device: VkCreateDevice,
    enumerate_device_extension_properties: VkEnumerateDeviceExtensionProperties,
}

impl PhysicalDeviceFunctions {
    /// Loads functions for physical devices
    pub(super) fn new(instance: VkInstance) -> Result<Self, &'static CStr> {
        let get_physical_device_properties: VkGetPhysicalDeviceProperties =
            get_instance_proc_addr!(instance, VK_GET_PHYSICAL_DEVICE_PROPERTIES)?;
        let get_physical_device_features: VkGetPhysicalDeviceFeatures =
            get_instance_proc_addr!(instance, VK_GET_PHYSICAL_DEVICE_FEATURES)?;
        let get_physical_device_queue_family_properties: VkGetPhysicalDeviceQueueFamilyProperties =
            get_instance_proc_addr!(instance, VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES)?;
        let create_device: VkCreateDevice = get_instance_proc_addr!(instance, VK_CREATE_DEVICE)?;
        let enumerate_device_extension_properties =
            get_instance_proc_addr!(instance, VK_ENUMERATE_DEVICE_EXTENSION_PROPERTIES)?;

        Ok(PhysicalDeviceFunctions {
            get_physical_device_properties,
            get_physical_device_features,
            get_physical_device_queue_family_properties,
            create_device,
            enumerate_device_extension_properties,
        })
    }

    /// Returns properties of a physical device
    pub(crate) fn get_physical_device_properties(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> VkPhysicalDeviceProperties {
        let mut properties = VkPhysicalDeviceProperties::default();
        (self.get_physical_device_properties)(physical_device, &mut properties);
        properties
    }

    /// Reports capabilities of a physical device
    pub(crate) fn get_physical_device_features(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> VkPhysicalDeviceFeatures {
        let mut features = VkPhysicalDeviceFeatures::default();
        (self.get_physical_device_features)(physical_device, &mut features);
        features
    }

    /// Reports properties of the queues of the specified physical device
    pub(crate) fn get_physical_device_queue_family_properties(
        &self,
        physical_device: VkPhysicalDevice,
        queue_family_property_count: &mut u32,
        queue_family_properties: *mut VkQueueFamilyProperties,
    ) {
        (self.get_physical_device_queue_family_properties)(
            physical_device,
            queue_family_property_count,
            queue_family_properties,
        );
    }

    /// Create a new device instance
    pub(crate) fn create_device(
        &self,
        physical_device: VkPhysicalDevice,
        create_info: &VkDeviceCreateInfo,
        allocator: *const VkAllocationCallbacks,
    ) -> Result<VkDevice, VkResult> {
        let mut device = null_mut();
        try_vulkan!((self.create_device)(
            physical_device,
            create_info,
            allocator,
            &mut device
        ))
        .map(|_| device)
    }

    /// Returns up to requested number of device extension properties
    pub(crate) fn enumerate_device_extension_properties(
        &self,
        physical_device: VkPhysicalDevice,
        layer_name: *const c_char,
        property_count: &mut u32,
        properties: *mut VkExtensionProperties,
    ) -> Result<(), VkResult> {
        try_vulkan!((self.enumerate_device_extension_properties)(
            physical_device,
            layer_name,
            property_count,
            properties
        ))
        .map(|_| ())
    }
}
