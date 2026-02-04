use crate::{Result, SharedObject, gpu::subsystem::GlobalVulkanFunctions, load_function};
use vulkan::{
    VK_CREATE_INSTANCE, VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES,
    VK_ENUMERATE_INSTANCE_LAYER_PROPERTIES, VK_ENUMERATE_INSTANCE_VERSION,
    VK_GET_INSTANCE_PROC_ADDR,
};

impl GlobalVulkanFunctions {
    /// Load the global Vulkan functions from `shared_object`
    pub fn load(shared_object: &SharedObject) -> Result<GlobalVulkanFunctions> {
        Ok(GlobalVulkanFunctions {
            get_instance_proc_addr: load_function!(shared_object, VK_GET_INSTANCE_PROC_ADDR)?,
            enumerate_instance_version: load_function!(
                shared_object,
                VK_ENUMERATE_INSTANCE_VERSION
            )?,
            enumerate_instance_extensions: load_function!(
                shared_object,
                VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES
            )?,
            enumerate_instance_layers: load_function!(
                shared_object,
                VK_ENUMERATE_INSTANCE_LAYER_PROPERTIES
            )?,
            create_instance: load_function!(shared_object, VK_CREATE_INSTANCE)?,
        })
    }
}
