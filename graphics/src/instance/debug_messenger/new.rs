use crate::{
    GraphicsDebugMessageSeverity, GraphicsDebugMessenger, GraphicsDebugMessengerCallback,
    GraphicsError, Result,
    instance::{debug_messenger::debug_message_trampoline, inner::GraphicsInstanceInner},
};
use std::{ptr::null, sync::Arc};
use vulkan::{
    ext_debug_utils::{
        VkDebugUtilsMessageTypeFlagExt, VkDebugUtilsMessengerCreateInfoExt,
        VkDebugUtilsMessengerExt,
    },
    try_vulkan,
};

impl<C: GraphicsDebugMessengerCallback> GraphicsDebugMessenger<C> {
    /// Create a new [`GraphicsDebugMessenger`]
    pub(in crate::instance) fn new(
        instance: Arc<GraphicsInstanceInner>,
        min_severity: GraphicsDebugMessageSeverity,
        callback: C,
    ) -> Result<GraphicsDebugMessenger<C>> {
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
            .functions
            .debug_messenger()
            .create_debug_messenger)(
            instance.handle(),
            &create_info,
            null(),
            &mut handle,
        ))
        .map_err(|error| GraphicsError::new_vk("unable to create debug messenger", error))?;

        Ok(GraphicsDebugMessenger {
            callback,
            handle,
            instance,
        })
    }
}
