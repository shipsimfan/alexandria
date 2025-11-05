//! User input subsystem

mod device;
mod events;
mod impls;

pub use device::*;
pub use events::*;
pub use impls::*;

/// A subsystem which can consume input events and present and interface for accessing them
pub trait Input {
    /// A new device has been connected
    ///
    /// Return a new ID to identify the input device for running
    fn device_connected(&mut self, device: InputDevice) -> InputDeviceId;

    /// A previously connected device has been disconnected
    ///
    /// Once the device has been disconnected, its ID can be re-used
    fn device_disconnected(&mut self, id: InputDeviceId);

    /// A button was pressed or released
    fn button_event(&mut self, event: InputButtonEvent);

    /// An axis was changed
    fn axis_event(&mut self, event: InputAxisEvent);
}
