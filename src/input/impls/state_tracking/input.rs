use crate::input::{
    Input, InputDevice, InputDeviceId, StateTrackingInput, StateTrackingInputDevice,
};

impl Input for StateTrackingInput {
    fn device_connected(&mut self, device: InputDevice) -> InputDeviceId {
        self.num_devices += 1;

        // Create device
        let device = StateTrackingInputDevice::new(device);

        // Search for slot
        for (i, slot) in self.input_devices.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(device);
                return i as u32;
            }
        }

        // No free slot, add it to the end
        let i = self.input_devices.len() as u32;
        self.input_devices.push(Some(device));
        i
    }

    fn device_disconnected(&mut self, id: InputDeviceId) {
        self.num_devices -= 1;
        self.input_devices[id as usize] = None;
    }

    fn axis_event(&mut self, event: crate::input::InputAxisEvent) {
        self.input_devices[event.id() as usize]
            .as_mut()
            .unwrap()
            .axis_event(event.axis(), event.value());
    }

    fn button_event(&mut self, event: crate::input::InputButtonEvent) {
        self.input_devices[event.id() as usize]
            .as_mut()
            .unwrap()
            .button_event(event.button(), event.pressed());
    }
}
