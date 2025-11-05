use crate::input::StateTrackingInputDevice;

impl StateTrackingInputDevice {
    /// Called to save the buttons states from the previous frame
    pub(in crate::input::impls::state_tracking) fn frame(&mut self) {
        self.prev_button_states.copy_from_slice(&self.button_states);
    }
}
