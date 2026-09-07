use crate::{
    Error, Result,
    gpu::{VulkanDevice, VulkanNameable},
};
use std::ffi::CStr;
use vulkan::{ext_debug_utils::VkDebugUtilsObjectNameInfoExt, try_vulkan};

impl VulkanDevice {
    /// Set the name of a Vulkan object
    pub fn set_object_name<T: VulkanNameable>(&self, object: &mut T, name: &CStr) -> Result<()> {
        try_vulkan!((self
            .functions()
            .set_debug_utils_object_name
            .as_ref()
            .unwrap())(
            self.handle(),
            &VkDebugUtilsObjectNameInfoExt {
                object_type: T::OBJECT_TYPE,
                object_handle: object.handle(),
                object_name: name.as_ptr(),
                ..Default::default()
            },
        ))
        .map(|_| ())
        .map_err(|error| Error::new_with("unable to set an object's name", error))
    }
}
