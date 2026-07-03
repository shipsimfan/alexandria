use crate::window::subsystem::linux::wayland::seat_listener::XkbState;
use xkbcommon::xkb_state_update_mask;

impl XkbState {
    /// Add a modifier to the state
    pub fn add_modifier(
        &mut self,
        depressed_mods: u32,
        latched_mods: u32,
        locked_mods: u32,
        depressed_layout: u32,
        latched_layout: u32,
        locked_layout: u32,
    ) {
        unsafe {
            xkb_state_update_mask(
                self.state,
                depressed_mods,
                latched_mods,
                locked_mods,
                depressed_layout,
                latched_layout,
                locked_layout,
            )
        };
    }
}
