use crate::{
    GraphicsInstanceExtension, Result,
    instance::{
        GraphicsAdapterFunctions, GraphicsDebugMessengerFunctions, inner::GraphicsInstanceFunctions,
    },
    util::load_instance_function,
};
use vulkan::{VK_DESTROY_INSTANCE, VK_ENUMERATE_PHYSICAL_DEVICES, VkInstance};

impl GraphicsInstanceFunctions {
    /// Load all the required instance functions
    pub fn load(
        instance: VkInstance,
        extensions: &[GraphicsInstanceExtension],
    ) -> Result<GraphicsInstanceFunctions> {
        let mut debug_messenger = None;
        for extension in extensions {
            if *extension == GraphicsInstanceExtension::DebugUtils {
                debug_messenger = Some(GraphicsDebugMessengerFunctions::load(instance)?);
            }
        }

        Ok(GraphicsInstanceFunctions {
            adapter: GraphicsAdapterFunctions::load(instance)?,
            debug_messenger,

            enumerate_physical_devices: load_instance_function!(
                instance,
                VK_ENUMERATE_PHYSICAL_DEVICES
            )?,
            destroy_instance: load_instance_function!(instance, VK_DESTROY_INSTANCE)?,
        })
    }
}
