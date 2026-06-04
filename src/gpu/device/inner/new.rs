use crate::{
    Error, Result,
    gpu::{
        VulkanAdapter, VulkanAdapterFeature, VulkanDeviceExtension, VulkanQueueCreateInfo,
        device::{VulkanDeviceFunctions, VulkanDeviceInner},
    },
};
use std::ptr::null;
use vulkan::{VkDevice, VkDeviceCreateInfo, try_vulkan, util::create_next_chain_mut};

impl VulkanDeviceInner {
    /// Create a new [`VulkanDeviceInner`]
    pub fn new(
        features: &mut [&mut dyn VulkanAdapterFeature],
        queues: &[VulkanQueueCreateInfo],
        extensions: &[VulkanDeviceExtension],
        adapter: &VulkanAdapter,
    ) -> Result<VulkanDeviceInner> {
        // Setup the next chain for features
        let next = create_next_chain_mut(features.into_iter().map(|feature| *feature as _));

        // Convert extensions to C strings
        let mut extension_ptrs = Vec::with_capacity(extensions.len());
        for extension in extensions {
            extension_ptrs.push(extension.as_cstr().as_ptr());
        }

        let create_info = VkDeviceCreateInfo {
            next,
            queue_create_info_count: queues.len() as _,
            queue_create_infos: queues.as_ptr().cast(),
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
