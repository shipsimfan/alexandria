use std::ffi::c_char;
use vulkan::bindings::VK_KHR_SURFACE_EXTENSION_NAME;

const EXTENSIONS: &[*const c_char] = &[VK_KHR_SURFACE_EXTENSION_NAME.as_ptr()];

pub fn get_extension_list() -> Vec<*const c_char> {
    Vec::from(EXTENSIONS)
}
