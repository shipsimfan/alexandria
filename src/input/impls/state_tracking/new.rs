use crate::input::StateTrackingInput;

impl StateTrackingInput {
    /// Create a new [`StateTrackingInput`]
    pub fn new() -> Self {
        StateTrackingInput {
            input_devices: Vec::with_capacity(8),
            num_devices: 0,
        }
    }
}
