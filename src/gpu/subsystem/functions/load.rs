use crate::{
    FunctionSymbol, Result, SharedObject,
    gpu::{load_global_function, subsystem::GlobalVulkanFunctions},
    load_function,
};
use vulkan::{
    VK_CREATE_INSTANCE, VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES,
    VK_ENUMERATE_INSTANCE_LAYER_PROPERTIES, VK_ENUMERATE_INSTANCE_VERSION,
    VK_GET_INSTANCE_PROC_ADDR, VkGetInstanceProcAddr,
};

impl GlobalVulkanFunctions {
    /// Load the global Vulkan functions from `shared_object`
    pub(in crate::gpu::subsystem) fn load(
        shared_object: &SharedObject,
    ) -> Result<GlobalVulkanFunctions> {
        let get_instance_proc_addr: FunctionSymbol<VkGetInstanceProcAddr> =
            load_function!(shared_object, VK_GET_INSTANCE_PROC_ADDR)?;

        Ok(GlobalVulkanFunctions {
            enumerate_instance_version: load_global_function!(
                get_instance_proc_addr,
                VK_ENUMERATE_INSTANCE_VERSION
            )?,
            enumerate_instance_extensions: load_global_function!(
                get_instance_proc_addr,
                VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES
            )?,
            enumerate_instance_layers: load_global_function!(
                get_instance_proc_addr,
                VK_ENUMERATE_INSTANCE_LAYER_PROPERTIES
            )?,
            create_instance: load_global_function!(get_instance_proc_addr, VK_CREATE_INSTANCE)?,
            get_instance_proc_addr,
        })
    }
}
