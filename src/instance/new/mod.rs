use crate::Instance;
use check_extension_support::check_extension_support;
use check_layer_support::check_layer_support;
use std::{ptr::null_mut, sync::Arc};
use vk::{os_instance_extensions, DebugUtilsMessenger, EventCallback, Global};

mod check_extension_support;
mod check_layer_support;
mod error;

pub use error::InstanceCreateError;

impl Instance {
    /// Creates a new [`Instance`]
    ///
    /// The parameters passed to the `event_callback` are:
    ///  * `severity` - The [`Severity`] of the message
    ///  * `message` - The text describing the message
    ///  * `objects` - A list of names of objects related to this message
    pub fn new(
        event_callback: Option<Box<dyn EventCallback>>,
    ) -> Result<Arc<Self>, InstanceCreateError> {
        let debug = event_callback.is_some();
        let event_callback = event_callback
            .map(|callback| Box::into_raw(Box::new(callback)))
            .unwrap_or(null_mut());

        // Check layer and extension support and create the instance
        let global = Global::new().map_err(InstanceCreateError::GlobalFunctionMissing)?;

        let layers = check_layer_support(&global, debug)?;
        let extensions = check_extension_support(os_instance_extensions(), &global, debug)?;

        let instance = vk::Instance::new(&global, layers, &extensions, event_callback)
            .map_err(InstanceCreateError::CreateInstanceFailed)?;

        // Create the debug messenger
        let debug_messenger = if debug {
            Some(
                DebugUtilsMessenger::new(instance.clone())
                    .map_err(InstanceCreateError::CreateDebugMessengerFailed)?,
            )
        } else {
            None
        };

        Ok(Arc::new(Instance {
            instance,
            debug_messenger,
        }))
    }
}
