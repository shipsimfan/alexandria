use super::debug_messenger_create_info;
use crate::{DebugUtilsMessenger, Instance};
use std::{borrow::Cow, ptr::null, rc::Rc};
use util::Severity;
use vulkan::VkResult;

impl DebugUtilsMessenger {
    /// Creates a new [`DebugUtilsMessenger`]
    ///
    /// The parameters passed to the `callback` are:
    ///  * `severity` - The [`Severity`] of the message
    ///  * `message` - The text describing the message
    ///  * `objects` - A list of names of objects related to this message
    pub fn new(
        instance: Rc<Instance>,
        callback: Box<dyn Fn(Severity, &str, Vec<Cow<str>>)>,
    ) -> Result<Self, VkResult> {
        let callback = Box::into_raw(Box::new(callback));

        let create_info = debug_messenger_create_info(callback);
        let handle =
            instance
                .f()
                .du()
                .unwrap()
                .create_messenger(instance.handle(), &create_info, null())?;

        Ok(DebugUtilsMessenger {
            handle,
            callback,
            instance,
        })
    }
}
