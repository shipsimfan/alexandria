use super::debug_messenger_create_info;
use crate::{DebugUtilsMessenger, Instance};
use std::{ptr::null, rc::Rc};
use vulkan::VkResult;

impl DebugUtilsMessenger {
    /// Creates a new [`DebugUtilsMessenger`]
    ///
    /// The parameters passed to the `callback` are:
    ///  * `severity` - The [`Severity`] of the message
    ///  * `message` - The text describing the message
    ///  * `objects` - A list of names of objects related to this message
    pub fn new(instance: Rc<Instance>) -> Result<Self, VkResult> {
        let create_info = debug_messenger_create_info(instance.event_callback);
        let handle =
            instance
                .f()
                .du()
                .unwrap()
                .create_messenger(instance.handle(), &create_info, null())?;

        Ok(DebugUtilsMessenger { handle, instance })
    }
}
