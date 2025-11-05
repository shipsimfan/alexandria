use crate::input::{StateTrackingInput, StateTrackingInputLogCallbacks};

impl<LogCallbacks: StateTrackingInputLogCallbacks> StateTrackingInput<LogCallbacks> {
    /// Create a new [`StateTrackingInput`]
    pub fn new(log_callbacks: LogCallbacks) -> Self {
        StateTrackingInput {
            input_devices: Vec::with_capacity(8),
            num_devices: 0,
            log_callbacks,
        }
    }
}
