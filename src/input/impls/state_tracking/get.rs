use crate::input::{StateTrackingInput, StateTrackingInputDevice};

impl StateTrackingInput {
    /// Get the number of registered devices
    pub fn num_devices(&self) -> usize {
        self.num_devices
    }

    /// Get an iterator over the devices with their IDs
    pub fn devices(&self) -> impl Iterator<Item = (usize, &StateTrackingInputDevice)> {
        self.input_devices
            .iter()
            .enumerate()
            .filter_map(|(id, device)| device.as_ref().map(|device| (id, device)))
    }

    /// Get a device at `index`
    pub fn device(&self, index: usize) -> &StateTrackingInputDevice {
        self.input_devices[index].as_ref().unwrap()
    }
}
