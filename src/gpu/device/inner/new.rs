use crate::{
    Error, Result,
    gpu::{
        VulkanAdapter, VulkanDeviceExtendedCreateInfo, VulkanDeviceExtension,
        VulkanQueueCreateInfo,
        device::{VulkanDeviceFunctions, VulkanDeviceInner},
    },
};
use std::ptr::null;
use vulkan::{VkDevice, VkDeviceCreateInfo, try_vulkan};

impl VulkanDeviceInner {
    /// Create a new [`VulkanDeviceInner`]
    pub fn new(
        extended_info: &[VulkanDeviceExtendedCreateInfo],
        queues: &[VulkanQueueCreateInfo],
        extensions: &[VulkanDeviceExtension],
        adapter: &VulkanAdapter,
    ) -> Result<VulkanDeviceInner> {
        // Convert extended info to Vulkan
        let mut extended_info: Vec<_> = extended_info
            .into_iter()
            .map(|extended_info| extended_info.to_vk())
            .collect();

        let next = if extended_info.len() > 0 {
            for i in 0..extended_info.len() - 1 {
                let (left, right) = extended_info.split_at_mut(i + 1);
                left[i].set_next(&mut right[0]);
            }
            extended_info[0].as_mut_ptr()
        } else {
            null()
        };

        // Convert queues to Vulkan
        let queues: Vec<_> = queues.into_iter().map(|queue| queue.to_vk()).collect();

        // Convert extensions to C strings
        let mut extension_ptrs = Vec::with_capacity(extensions.len());
        for extension in extensions {
            extension_ptrs.push(extension.as_cstr().as_ptr());
        }

        let create_info = VkDeviceCreateInfo {
            next,
            queue_create_info_count: queues.len() as _,
            queue_create_infos: queues.as_ptr(),
            enabled_extension_count: extension_ptrs.len() as _,
            enabled_extension_names: extension_ptrs.as_ptr(),
            ..Default::default()
        };

        let mut handle = VkDevice::null();
        try_vulkan!((adapter.instance().functions().create_device)(
            adapter.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|vk| Error::new_with("unable to create graphics device", vk))?;

        let functions = VulkanDeviceFunctions::load(adapter.instance(), handle, extensions)?;

        Ok(VulkanDeviceInner {
            handle,
            _instance: adapter.instance().clone(),
            functions,
        })
    }
}
