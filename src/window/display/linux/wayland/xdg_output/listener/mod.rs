use std::ffi::CStr;

mod trampolines;

/// An item which can be used at the callback to XDG output events
pub(in crate::window) trait XdgOutputListener: Sized {
    /// Called when the logical position of the output changes
    fn logical_position(&mut self, x: i32, y: i32);

    /// Called when the logical size of the output changes
    fn logical_size(&mut self, width: i32, height: i32);

    /// Called when the output name is advertised or changed
    fn name(&mut self, name: &CStr);

    /// Called when the output description is advertised or changed
    fn description(&mut self, description: &CStr);
}
