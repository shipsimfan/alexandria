use crate::Instance;
use check_extension_support::check_extension_support;
use check_layer_support::check_layer_support;
use std::borrow::Cow;
use util::Severity;
use vk::{DebugUtilsMessenger, Global, InstanceExtension};

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
        event_callback: Option<impl Fn(Severity, &str, Vec<Cow<str>>) + 'static>,
    ) -> Result<Self, InstanceCreateError> {
        let event_callback = event_callback
            .map(|callback| Box::new(callback) as Box<dyn Fn(Severity, &str, Vec<Cow<str>>)>);

        // Check layer and extension support and create the instance
        let global = Global::new().map_err(InstanceCreateError::GlobalFunctionMissing)?;

        let layers = check_layer_support(&global, event_callback.is_some())?;
        let extensions =
            check_extension_support(InstanceExtension::os(), &global, event_callback.is_some())?;

        let instance = vk::Instance::new(&global, layers, &extensions, event_callback.is_some())
            .map_err(InstanceCreateError::CreateInstanceFailed)?;

        // Create the debug messenger
        let debug_messenger = match event_callback {
            Some(callback) => Some(
                DebugUtilsMessenger::new(instance.clone(), callback)
                    .map_err(InstanceCreateError::CreateDebugMessengerFailed)?,
            ),
            None => None,
        };

        Ok(Instance {
            instance,
            debug_messenger,
        })
    }
}
