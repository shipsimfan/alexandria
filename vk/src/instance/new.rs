use crate::{
    functions::InstanceFunctions, CreateError, EventCallback, Global, Instance, InstanceExtension,
    InstanceLayer,
};
use std::{
    ptr::{null, null_mut},
    rc::Rc,
};
use vulkan::{VkApplicationInfo, VkInstanceCreateInfo};

use super::debug_messenger::debug_messenger_create_info;

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
        event_callback: *mut Box<dyn EventCallback>,
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

        let debug_messenger_create_info = if event_callback != null_mut() {
            Some(debug_messenger_create_info(event_callback))
        } else {
            None
        };

        let create_info = VkInstanceCreateInfo {
            application_info: &application_info,
            enabled_layer_count: layers.len() as _,
            enabled_layer_names: layers.as_ptr(),
            enabled_extension_count: extensions.len() as _,
            enabled_extension_names: extensions.as_ptr(),
            next: debug_messenger_create_info
                .as_ref()
                .map(|info| info as *const _ as _)
                .unwrap_or(null_mut()),
            ..Default::default()
        };

        let handle = global.f().create_instance(&create_info, null())?;
        let functions = InstanceFunctions::new(handle, event_callback != null_mut())?;

        Ok(Rc::new(Instance {
            handle,
            functions,
            event_callback,
        }))
    }
}
