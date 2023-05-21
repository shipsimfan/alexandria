const SURFACE_EXTENSION: &str = "VK_KHR_surface\0";
const WIN32_SURFACE_EXTENSION: &str = "VK_KHR_win32_surface\0";

const EXTENSIONS: &[*const u8] = &[SURFACE_EXTENSION.as_ptr(), WIN32_SURFACE_EXTENSION.as_ptr()];

pub fn get_extension_list() -> &'static [*const u8] {
    EXTENSIONS
}
