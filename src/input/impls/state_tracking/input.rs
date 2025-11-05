use crate::input::{
    Input, InputAxisEvent, InputButtonEvent, InputDevice, InputDeviceId, StateTrackingInput,
    StateTrackingInputDevice, StateTrackingInputLogCallbacks,
};

impl<LogCallbacks: StateTrackingInputLogCallbacks> Input for StateTrackingInput<LogCallbacks> {
    fn device_connected(&mut self, device: InputDevice) -> InputDeviceId {
        self.num_devices += 1;

        // Create device
        let device = StateTrackingInputDevice::new(device);

        // Search for slot
        for (i, slot) in self.input_devices.iter_mut().enumerate() {
            if slot.is_none() {
                self.log_callbacks.on_device_connected(i, &device);
                *slot = Some(device);
                return i as u32;
            }
        }

        // No free slot, add it to the end
        let i = self.input_devices.len();
        self.log_callbacks.on_device_connected(i, &device);
        self.input_devices.push(Some(device));
        i as u32
    }

    fn device_disconnected(&mut self, id: InputDeviceId) {
        self.log_callbacks
            .on_device_disconnected(id as _, self.input_devices[id as usize].as_ref().unwrap());
        self.num_devices -= 1;
        self.input_devices[id as usize] = None;
    }

    fn axis_event(&mut self, event: InputAxisEvent) {
        let device = self.input_devices[event.id() as usize].as_mut().unwrap();
        device.axis_event(event.axis(), event.value());
        self.log_callbacks.on_axis_event(&event, device);
    }

    fn button_event(&mut self, event: InputButtonEvent) {
        let device = self.input_devices[event.id() as usize].as_mut().unwrap();
        device.button_event(event.button(), event.pressed());
        self.log_callbacks.on_button_event(&event, device);
    }

    fn frame(&mut self) {
        for device in &mut self.input_devices {
            if let Some(device) = device {
                device.frame();
            }
        }
    }
}
