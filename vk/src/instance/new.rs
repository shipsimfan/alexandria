use crate::{
    functions::InstanceFunctions, CreateError, Global, Instance, InstanceExtension, InstanceLayer,
};
use std::{ptr::null, rc::Rc};
use vulkan::{VkApplicationInfo, VkInstanceCreateInfo};

impl Instance {
    /// Creates a new [`Instance`]
    ///
    /// The parameters passed to the `callback` are:
    ///  * `severity` - The [`Severity`] of the message
    ///  * `message` - The text describing the message
    ///  * `objects` - A list of names of objects related to this message
    pub fn new(
        global: &Global,
        layers: &[InstanceLayer],
        extensions: &[InstanceExtension],
        debug: bool,
    ) -> Result<Rc<Self>, CreateError> {
        let layers: Vec<_> = layers
            .into_iter()
            .map(|layer| layer.as_cstr().as_ptr())
            .collect();

        let extensions: Vec<_> = extensions
            .into_iter()
            .map(|extension| extension.as_cstr().as_ptr())
            .collect();

        let application_info = VkApplicationInfo {
            ..Default::default()
        };

        let create_info = VkInstanceCreateInfo {
            application_info: &application_info,
            enabled_layer_count: layers.len() as _,
            enabled_layer_names: layers.as_ptr(),
            enabled_extension_count: extensions.len() as _,
            enabled_extension_names: extensions.as_ptr(),
            ..Default::default()
        };

        let handle = global.f().create_instance(&create_info, null())?;
        let functions = InstanceFunctions::new(handle, debug)?;

        Ok(Rc::new(Instance { handle, functions }))
    }
}
