use crate::{
    Error, Result,
    gpu::{
        VulkanDebugMessageSeverity, VulkanDebugMessenger, VulkanDebugMessengerCallback,
        VulkanInstance, instance::debug_messenger::debug_message_trampoline,
    },
};
use std::ptr::null;
use vulkan::{
    ext_debug_utils::{
        VkDebugUtilsMessageTypeFlagExt, VkDebugUtilsMessengerCreateInfoExt,
        VkDebugUtilsMessengerExt,
    },
    try_vulkan,
};

impl<C: VulkanDebugMessengerCallback> VulkanDebugMessenger<C> {
    /// Create a new [`VulkanDebugMessenger`]
    pub(in crate::gpu::instance) fn new(
        instance: VulkanInstance,
        min_severity: VulkanDebugMessageSeverity,
        callback: C,
    ) -> Result<VulkanDebugMessenger<C>> {
        let callback = Box::new(callback);

        let mut create_info = VkDebugUtilsMessengerCreateInfoExt {
            message_type: VkDebugUtilsMessageTypeFlagExt::GeneralBitExt
                | VkDebugUtilsMessageTypeFlagExt::ValidationBitExt
                | VkDebugUtilsMessageTypeFlagExt::PerformanceBitExt,
            user_callback: debug_message_trampoline::<C>,
            user_data: Box::as_ptr(&callback).cast_mut().cast(),
            ..Default::default()
        };

        create_info.message_severity = min_severity.to_vk();

        let mut handle = VkDebugUtilsMessengerExt::null();
        try_vulkan!((instance
            .functions()
            .debug_messenger()
            .create_debug_messenger)(
            instance.handle(),
            &create_info,
            null(),
            &mut handle,
        ))
        .map_err(|error| Error::new_with("unable to create debug messenger", error))?;

        Ok(VulkanDebugMessenger {
            callback,
            handle,
            instance,
        })
    }
}
