use crate::{
    MemorySize, Uuid,
    gpu::{VulkanAdapter, VulkanAdapterKind, VulkanInstance, VulkanQueueFamilyInfo, VulkanVersion},
};
use std::{borrow::Cow, ffi::CStr, ptr::null_mut};
use vulkan::{
    VkMemoryHeapFlag, VkPhysicalDevice, VkPhysicalDeviceMemoryProperties,
    VkPhysicalDeviceProperties,
};

impl<'instance> VulkanAdapter<'instance> {
    /// Create a new [`VulkanAdapter`]
    pub(in crate::gpu::instance) fn new(
        instance: &'instance VulkanInstance,
        handle: VkPhysicalDevice,
    ) -> VulkanAdapter<'instance> {
        // Get the properties
        let mut properties = VkPhysicalDeviceProperties::default();
        unsafe {
            (instance.functions.adapter.get_physical_device_properties)(handle, &mut properties)
        };

        // Extract the properties
        let api_version = unsafe { VulkanVersion::new_raw(properties.api_version) };
        let driver_version = unsafe { VulkanVersion::new_raw(properties.driver_version) };

        let kind = VulkanAdapterKind::from_vk(properties.device_type);

        let name_c = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) };
        let name = match name_c.to_string_lossy() {
            Cow::Owned(owned) => owned,
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
        };

        let uuid = Uuid::from_flat(properties.pipeline_cache_uuid);

        // Get memory information
        let mut memory_information = VkPhysicalDeviceMemoryProperties::default();
        unsafe {
            (instance
                .functions
                .adapter
                .get_physical_device_memory_properties)(handle, &mut memory_information)
        };

        // Calculate total VRAM
        let mut vram = 0;
        for i in 0..memory_information.memory_heap_count as usize {
            let heap = &memory_information.memory_heaps[i];
            if heap.flags.contains(VkMemoryHeapFlag::DeviceLocalBit) {
                vram += heap.size;
            }
        }
        let vram = MemorySize::new(vram);

        // Enumerate queue families
        let mut num_queue_families = 0;
        unsafe {
            (instance
                .functions
                .adapter
                .get_physical_device_queue_family_properties)(
                handle,
                &mut num_queue_families,
                null_mut(),
            )
        };

        let mut queue_families = Vec::with_capacity(num_queue_families as _);
        unsafe {
            (instance
                .functions
                .adapter
                .get_physical_device_queue_family_properties)(
                handle,
                &mut num_queue_families,
                queue_families.as_mut_ptr(),
            )
        };
        unsafe { queue_families.set_len(num_queue_families as _) };
        let queue_families = queue_families
            .into_iter()
            .enumerate()
            .filter_map(|(index, info)| VulkanQueueFamilyInfo::new(index as _, info))
            .collect();

        VulkanAdapter {
            handle,
            api_version,
            driver_version,
            kind,
            name,
            uuid,
            vram,
            queue_families,
            instance,
        }
    }
}
