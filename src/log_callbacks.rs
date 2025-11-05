use crate::{
    input::{
        InputAxisEvent, InputButtonEvent, InputDeviceKind, KeyCode, StateTrackingInputDevice,
        StateTrackingInputLogCallbacks,
    },
    math::Vector2u,
};

/// A severity that a log message reported by the underlying graphics API can be
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphicsApiLogSeverity {
    /// An error occurred in the underlying graphics API
    Error,

    /// A warning was emitted by the underlying graphics API
    Warning,

    /// General information or debugging message was emitted by the underlying graphics API
    Info,
}

/// Functions that are called by Alexandria when certain events occur, intended to be used to log
/// those events.
pub trait LogCallbacks {
    /// Called when the window is resized
    #[allow(unused_variables)]
    fn on_resize(&mut self, size: Vector2u) {}

    /// Called when the underlying graphics API emits a log message
    #[allow(unused_variables)]
    fn on_graphics_api_log(&mut self, severity: GraphicsApiLogSeverity, message: String) {}

    /// Called when the keyboard produces a `scan_code` that couldn't be translated to a
    /// [`crate::input::KeyCode`]
    #[allow(unused_variables)]
    fn on_unknown_scan_code(&mut self, scan_code: u16, pressed: bool) {}
}

impl LogCallbacks for () {}

/// An example logger which prints to standard output
pub struct StdoutLogger;

impl LogCallbacks for StdoutLogger {
    fn on_resize(&mut self, size: Vector2u) {
        println!("Window resized | New size: {}x{}", size.x, size.y);
    }

    fn on_graphics_api_log(&mut self, severity: GraphicsApiLogSeverity, message: String) {
        let severity = match severity {
            GraphicsApiLogSeverity::Error => "Error",
            GraphicsApiLogSeverity::Warning => "Warning",
            GraphicsApiLogSeverity::Info => "Info",
        };
        println!("Graphics API | {}: {}", severity, message);
    }

    fn on_unknown_scan_code(&mut self, scan_code: u16, pressed: bool) {
        println!(
            "Input | Unknown scancode {} {}",
            scan_code,
            if pressed { "pressed" } else { "released" }
        );
    }
}

impl StateTrackingInputLogCallbacks for StdoutLogger {
    fn on_device_connected(&mut self, id: usize, device: &StateTrackingInputDevice) {
        println!(
            "Input | Device \"{}\" registered with id {}",
            device.name(),
            id
        );
    }

    fn on_device_disconnected(&mut self, id: usize, device: &StateTrackingInputDevice) {
        println!(
            "Input | Device \"{}\" unregistered from id {}",
            id,
            device.name()
        );
    }

    fn on_axis_event(&mut self, event: &InputAxisEvent, _: &StateTrackingInputDevice) {
        println!(
            "Input | Axis {} on device {} set to {}",
            event.axis(),
            event.id(),
            event.value()
        );
    }

    fn on_button_event(&mut self, event: &InputButtonEvent, device: &StateTrackingInputDevice) {
        if device.kind() == InputDeviceKind::Keyboard {
            println!(
                "Input | Key [{}] on device {} {}",
                KeyCode::from_button(event.button()).unwrap(),
                event.id(),
                if event.pressed() {
                    "pressed"
                } else {
                    "released"
                }
            );
        } else {
            println!(
                "Input | Button {} on device {} {}",
                event.button(),
                event.id(),
                if event.pressed() {
                    "pressed"
                } else {
                    "released"
                }
            );
        }
    }
}
