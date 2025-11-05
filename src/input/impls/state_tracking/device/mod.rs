use crate::input::{InputDeviceKind, InputDeviceMetadata};

mod event;
mod frame;
mod get;
mod new;

/// The information for tracking the state of a single input device
pub struct StateTrackingInputDevice {
    /// The kind of input device this is
    kind: InputDeviceKind,

    /// The name of this device
    name: String,

    /// The metadata to identify the kind of device
    metadata: InputDeviceMetadata,

    /// The current states of all buttons of the device
    button_states: Vec<bool>,

    /// The previous states of all buttons of the device
    prev_button_states: Vec<bool>,

    /// The current states of all axes of the device
    axis_states: Vec<f32>,
}
