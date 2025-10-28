use crate::math::Vector2u;

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
}
