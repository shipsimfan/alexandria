use crate::{
    Error, Result,
    gpu::{
        GpuSubsystem, VulkanInstanceExtension, VulkanInstanceFunctions, VulkanVersion,
        instance::VulkanInstanceInner,
    },
};
use std::{borrow::Cow, ffi::CString, ptr::null, str::FromStr};
use vulkan::{VkApplicationInfo, VkInstance, VkInstanceCreateInfo, try_vulkan};

impl VulkanInstanceInner {
    /// Create a new [`VulkanInstanceInner`]
    pub fn new(
        context: &GpuSubsystem,
        api_version: VulkanVersion,
        application: Option<(&str, VulkanVersion)>,
        engine: Option<(&str, VulkanVersion)>,
        extensions: &[VulkanInstanceExtension],
        layers: &[Cow<str>],
    ) -> Result<VulkanInstanceInner> {
        // Create application info
        let mut application_info = VkApplicationInfo {
            api_version: api_version.as_vk(),
            ..Default::default()
        };

        let _application_name = application.map(|(name, version)| {
            let name = CString::from_str(name).unwrap();
            application_info.application_name = name.as_ptr();
            application_info.application_version = version.as_vk();
            name
        });

        let _engine_name = engine.as_ref().map(|(name, version)| {
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

        let mut layer_cstrs = Vec::with_capacity(layers.len());
        let mut layer_ptrs = Vec::with_capacity(layers.len());
        for layer in layers {
            let layer_cstr = CString::from_str(layer).unwrap();
            layer_ptrs.push(layer_cstr.as_ptr());
            layer_cstrs.push(layer_cstr);
        }
        create_info.enabled_layer_count = layers.len() as _;
        create_info.enabled_layer_names = layer_ptrs.as_ptr();

        let mut extension_ptrs = Vec::with_capacity(extensions.len());
        for extension in extensions {
            extension_ptrs.push(extension.as_cstr().as_ptr());
        }
        create_info.enabled_extension_count = extensions.len() as _;
        create_info.enabled_extension_names = extension_ptrs.as_ptr();

        // Create the instance
        let mut handle = VkInstance::null();
        try_vulkan!((context.functions().create_instance)(
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|vk| Error::new_with("unable to create graphics instance", vk))?;

        let functions = VulkanInstanceFunctions::load(context, handle, extensions)?;

        Ok(VulkanInstanceInner {
            handle,
            functions,
            _context: context.clone(),
        })
    }
}
