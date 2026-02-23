use std::ffi::CStr;
use wayland::wl_output_transform;

mod add;
mod trampolines;

/// An item which can be used at the callback to output events
pub(in crate::window::display::linux::wayland) trait WlOutputListener:
    Sized
{
    /// Called when the output properties are advertised or changed
    fn geometry(
        &mut self,
        x: i32,
        y: i32,
        physical_width: i32,
        physical_height: i32,
        make: &CStr,
        model: &CStr,
        transform: wl_output_transform,
    );

    /// Called when a new mode is advertised for the output
    fn mode(&mut self, flags: i32, width: i32, height: i32, refresh: i32);

    /// Called when the output properties are advertised or changed after all other events have been sent
    fn done(&mut self);

    /// Called when the output scale factor changes
    fn scale(&mut self, factor: i32);

    /// Called when the output name is advertised or changed
    fn name(&mut self, name: &CStr);

    /// Called when the output description is advertised or changed
    fn description(&mut self, description: &CStr);
}
