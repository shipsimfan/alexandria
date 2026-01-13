use crate::{
    GraphicsAdapter, GraphicsAdapterKind, GraphicsQueueFamilyInfo, GraphicsVersion,
    instance::GraphicsInstanceInner,
};
use alexandria_util::{MemorySize, UUID};
use std::{borrow::Cow, ffi::CStr, ptr::null_mut};
use vulkan::{
    VkMemoryHeapFlag, VkPhysicalDevice, VkPhysicalDeviceMemoryProperties,
    VkPhysicalDeviceProperties,
};

impl<'instance> GraphicsAdapter<'instance> {
    /// Create a new [`GraphicsAdapter`]
    pub(in crate::instance) fn new(
        instance: &'instance GraphicsInstanceInner,
        handle: VkPhysicalDevice,
    ) -> GraphicsAdapter<'instance> {
        // Get the properties
        let mut properties = VkPhysicalDeviceProperties::default();
        (instance.functions.adapter.get_physical_device_properties)(handle, &mut properties);

        // Extract the properties
        let api_version = unsafe { GraphicsVersion::new_raw(properties.api_version) };
        let driver_version = unsafe { GraphicsVersion::new_raw(properties.driver_version) };

        let kind = GraphicsAdapterKind::from_vk(properties.device_type);

        let name_c = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) };
        let name = match name_c.to_string_lossy() {
            Cow::Owned(owned) => owned,
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
        };

        let uuid = UUID::from_flat(properties.pipeline_cache_uuid);

        // Get memory information
        let mut memory_information = VkPhysicalDeviceMemoryProperties::default();
        (instance
            .functions
            .adapter
            .get_physical_device_memory_properties)(handle, &mut memory_information);

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
        (instance
            .functions
            .adapter
            .get_physical_device_queue_family_properties)(
            handle,
            &mut num_queue_families,
            null_mut(),
        );

        let mut queue_families = Vec::with_capacity(num_queue_families as _);
        (instance
            .functions
            .adapter
            .get_physical_device_queue_family_properties)(
            handle,
            &mut num_queue_families,
            queue_families.as_mut_ptr(),
        );
        unsafe { queue_families.set_len(num_queue_families as _) };
        let queue_families = queue_families
            .into_iter()
            .enumerate()
            .filter_map(|(index, info)| GraphicsQueueFamilyInfo::new(index as _, info))
            .collect();

        GraphicsAdapter {
            handle,
            api_version,
            driver_version,
            kind,
            name,
            uuid,
            vram,
            queue_families,
            _instance: instance,
        }
    }
}
