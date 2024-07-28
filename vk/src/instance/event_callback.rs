use std::borrow::Cow;
use util::Severity;

/// An object which can handle events emitted from Vulkan
pub trait EventCallback: 'static {
    /// The handler for events
    fn callback(&self, severity: Severity, message: &str, objects: Vec<Cow<str>>);
}

impl<F: for<'a, 'b> Fn(Severity, &'a str, Vec<Cow<'b, str>>) + 'static> EventCallback for F {
    fn callback(&self, severity: Severity, message: &str, objects: Vec<Cow<str>>) {
        self(severity, message, objects)
    }
}
