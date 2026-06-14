use crate::{
    Error, Result,
    window::subsystem::linux::wayland::seat_listener::{MMapRegion, XkbContext, XkbState},
};
use wayland::wl_keyboard_keymap_format;
use xkbcommon::{
    XkbKeymapCompileFlags, XkbKeymapFormat, xkb_keymap_new_from_buffer, xkb_keymap_unref,
    xkb_state_new,
};

impl XkbState {
    /// Create a new xkb state from the given keymap
    pub fn new(
        format: wl_keyboard_keymap_format,
        fd: i32,
        size: u32,
        xkb_context: &XkbContext,
    ) -> Result<Option<XkbState>> {
        // Open the memory mapped region
        let mem_region = match format {
            wl_keyboard_keymap_format::XkbV1 => MMapRegion::new(fd, size as _)?,
            _ => return Ok(None),
        };

        // Create the xkb keymap
        let keymap_slice = mem_region.as_slice();
        let keymap = unsafe {
            xkb_keymap_new_from_buffer(
                xkb_context.handle(),
                keymap_slice.as_ptr() as *const _,
                keymap_slice.len() as _,
                XkbKeymapFormat::TextV1,
                XkbKeymapCompileFlags::NoFlags,
            )
        };
        if keymap.is_null() {
            return Err(Error::new("unable to create xkb keymap"));
        }

        // Create the xkb state
        let state = unsafe { xkb_state_new(keymap) };
        if state.is_null() {
            unsafe { xkb_keymap_unref(keymap) };
            return Err(Error::new("unable to create xkb state"));
        }

        Ok(Some(XkbState {
            _mem_region: mem_region,
            keymap,
            state,
        }))
    }
}
