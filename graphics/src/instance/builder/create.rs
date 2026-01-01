use crate::{
    GraphicsError, GraphicsInstance, GraphicsInstanceBuilder, Result,
    instance::GraphicsInstanceFunctions, util::load_global_function,
};
use std::{ffi::CString, ptr::null, str::FromStr};
use vulkan::{
    VK_CREATE_INSTANCE, VkApplicationInfo, VkCreateInstance, VkInstance, VkInstanceCreateInfo,
    try_vulkan,
};

impl<'a> GraphicsInstanceBuilder<'a> {
    /// Create a new [`GraphicsInstance`] with the provided settings
    pub fn create(&self) -> Result<GraphicsInstance> {
        // Create application info
        let mut application_info = VkApplicationInfo {
            api_version: self.api_version.as_vk(),
            ..Default::default()
        };

        let _application_name = self.application.as_ref().map(|(name, version)| {
            let name = CString::from_str(name).unwrap();
            application_info.application_name = name.as_ptr();
            application_info.application_version = version.as_vk();
            name
        });

        let _engine_name = self.engine.as_ref().map(|(name, version)| {
            let name = CString::from_str(name).unwrap();
            application_info.engine_name = name.as_ptr();
            application_info.engine_version = version.as_vk();
            name
        });

        // Create the creation info
        let mut create_info = VkInstanceCreateInfo {
            application_info: &application_info,
            ..Default::default()
        };

        let mut layers = Vec::with_capacity(self.layers.len());
        let mut layer_ptrs = Vec::with_capacity(self.layers.len());
        for layer in &self.layers {
            let layer = CString::from_str(layer).unwrap();
            layer_ptrs.push(layer.as_ptr());
            layers.push(layer);
        }
        create_info.enabled_layer_count = self.layers.len() as _;
        create_info.enabled_layer_names = layer_ptrs.as_ptr();

        // Get the "vkCreateInstance" function
        let create_instance: VkCreateInstance = load_global_function!(VK_CREATE_INSTANCE)?;

        // Create the instance
        let mut instance = VkInstance::null();
        try_vulkan!(create_instance(&create_info, null(), &mut instance))
            .map_err(|vk| GraphicsError::new_vk("unable to create graphics instance", vk))?;

        // Load instance functions
        let functions = GraphicsInstanceFunctions::load(instance)?;

        Ok(GraphicsInstance {
            instance,
            functions,
        })
    }
}
