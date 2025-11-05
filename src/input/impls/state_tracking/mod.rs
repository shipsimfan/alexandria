mod device;
mod log_callbacks;

mod get;
mod input;
mod new;

pub use device::StateTrackingInputDevice;
pub use log_callbacks::StateTrackingInputLogCallbacks;

/// An input that tracks the state of inputs
pub struct StateTrackingInput<LogCallbacks: StateTrackingInputLogCallbacks = ()> {
    /// The devices that have been registered
    input_devices: Vec<Option<StateTrackingInputDevice>>,

    /// The current number of input devices
    num_devices: usize,

    /// The functions to call when events occur
    log_callbacks: LogCallbacks,
}
