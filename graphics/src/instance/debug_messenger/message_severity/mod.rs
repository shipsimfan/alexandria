mod as_str;
mod display;
mod from_vk;
mod to_vk;

/// A severity that a Vulkan debug message can be
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphicsDebugMessageSeverity {
    /// Most verbose output
    Verbose,

    /// Informational messages such as resource details
    Info,

    /// Specifies a use of Vulkan that may be an application bug
    Warning,

    /// Specifies an application has violated a valid usage condition
    Error,
}
