use crate::{GraphicsAdapter, GraphicsAdapterKind, GraphicsInstance, GraphicsVersion};
use alexandria_util::{MemorySize, UUID};
use std::{borrow::Cow, ffi::CStr};
use vulkan::{
    VkMemoryHeapFlag, VkPhysicalDevice, VkPhysicalDeviceMemoryProperties,
    VkPhysicalDeviceProperties,
};

impl<'instance> GraphicsAdapter<'instance> {
    /// Create a new [`GraphicsAdapter`]
    pub(in crate::instance) fn new(
        instance: &'instance GraphicsInstance,
        adapter: VkPhysicalDevice,
    ) -> GraphicsAdapter<'instance> {
        // Get the properties
        let mut properties = VkPhysicalDeviceProperties::default();
        (instance.functions.adapter.get_physical_device_properties)(adapter, &mut properties);

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
            .get_physical_device_memory_properties)(adapter, &mut memory_information);

        // Calculate total VRAM
        let mut vram = 0;
        for i in 0..memory_information.memory_heap_count as usize {
            let heap = &memory_information.memory_heaps[i];
            if heap.flags.contains(VkMemoryHeapFlag::DeviceLocalBit) {
                vram += heap.size;
            }
        }
        let vram = MemorySize::new(vram);

        GraphicsAdapter {
            adapter,
            api_version,
            driver_version,
            kind,
            name,
            uuid,
            vram,
            _instance: instance,
        }
    }
}
