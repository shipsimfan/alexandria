use crate::{
    Error, Result,
    gpu::{
        VulkanAdapter, VulkanDeviceExtension, VulkanExtendedAdapterInfo, VulkanQueueCreateInfo,
        device::{VulkanDeviceFunctions, VulkanDeviceInner},
    },
};
use std::ptr::null;
use vulkan::{VkDevice, VkDeviceCreateInfo, try_vulkan};

impl VulkanDeviceInner {
    /// Create a new [`VulkanDeviceInner`]
    pub fn new(
        extended_info: &mut [VulkanExtendedAdapterInfo],
        queues: &[VulkanQueueCreateInfo],
        extensions: &[VulkanDeviceExtension],
        adapter: &VulkanAdapter,
    ) -> Result<VulkanDeviceInner> {
        // Setup the next chain for extended information
        let next = VulkanExtendedAdapterInfo::set_next_chain(extended_info);

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
