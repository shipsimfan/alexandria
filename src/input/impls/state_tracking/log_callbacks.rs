use crate::input::{InputAxisEvent, InputButtonEvent, StateTrackingInputDevice};

/// Functions called when events occur in the input subsystem
pub trait StateTrackingInputLogCallbacks {
    /// Called when a new device is connected
    #[allow(unused_variables)]
    fn on_device_connected(&mut self, id: usize, device: &StateTrackingInputDevice) {}

    /// Called when a new device is disconnected
    #[allow(unused_variables)]
    fn on_device_disconnected(&mut self, id: usize, device: &StateTrackingInputDevice) {}

    /// Called when an axis value changes
    #[allow(unused_variables)]
    fn on_axis_event(&mut self, event: &InputAxisEvent, device: &StateTrackingInputDevice) {}

    /// Called when a button is pressed or released
    #[allow(unused_variables)]
    fn on_button_event(&mut self, event: &InputButtonEvent, device: &StateTrackingInputDevice) {}
}

impl StateTrackingInputLogCallbacks for () {}
