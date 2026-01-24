use crate::{
    GraphicsAdapter, GraphicsDeviceExtendedCreateInfo, GraphicsDeviceExtension, GraphicsError,
    GraphicsQueueCreateInfo, Result,
    device::{GraphicsDeviceInner, inner::functions::GraphicsDeviceFunctions},
};
use std::ptr::null;
use vulkan::{VkDevice, VkDeviceCreateInfo, try_vulkan};

impl GraphicsDeviceInner {
    /// Create a new [`GraphicsDeviceInner`]
    pub(in crate::device) fn new(
        extended_info: &[GraphicsDeviceExtendedCreateInfo],
        queues: &[GraphicsQueueCreateInfo],
        extensions: &[GraphicsDeviceExtension],
        adapter: &GraphicsAdapter,
    ) -> Result<GraphicsDeviceInner> {
        // Convert extended info to Vulkan
        let mut extended_info: Vec<_> = extended_info
            .into_iter()
            .map(|extended_info| extended_info.into_vk())
            .collect();

        let next = if extended_info.len() > 0 {
            for i in 0..extended_info.len() - 1 {
                let ptr = &extended_info[i + 1] as _;
                extended_info[i].set_next(ptr);
            }
            extended_info.as_ptr().cast()
        } else {
            null()
        };

        // Convert queues to Vulkan
        let queues: Vec<_> = queues.into_iter().map(|queue| queue.into_vk()).collect();

        // Convert extensions to C strings
        let mut extension_ptrs = Vec::with_capacity(extensions.len());
        for extension in extensions {
            extension_ptrs.push(extension.as_cstr().as_ptr());
        }

        let create_info = VkDeviceCreateInfo {
            next,
            queue_create_info_count: queues.len() as _,
            queue_create_infos: queues.as_ptr(),
            enabled_layer_count: extension_ptrs.len() as _,
            enabled_extension_names: extension_ptrs.as_ptr(),
            ..Default::default()
        };

        let mut handle = VkDevice::null();
        try_vulkan!((adapter.instance().functions.create_device)(
            adapter.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to create graphics device", vk))?;

        let functions = GraphicsDeviceFunctions::load(adapter.instance(), handle)?;

        Ok(GraphicsDeviceInner {
            handle,
            _instance: adapter.instance().clone(),
            functions,
        })
    }
}
