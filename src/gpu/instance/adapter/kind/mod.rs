mod as_str;
mod display;
mod from_vk;

/// The kind of a graphics adapter, ordered such that `better > worse`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VulkanAdapterKind {
    /// The adapter type is not one of the other types
    Other,

    /// The adapter is implemented on the CPU
    Cpu,

    /// The adapter is from a virtual machine
    Virtual,

    /// The adapter is integrated into the CPU
    Integrated,

    /// The adapter is discrete from the CPU
    Discrete,
}
