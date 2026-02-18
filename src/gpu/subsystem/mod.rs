use crate::define_handle;
use functions::GlobalVulkanFunctions;
use inner::GpuSubsystemInner;

mod functions;
mod inner;

mod all_extensions;
mod extensions;
mod get;
mod instance_builder;
mod layers;
mod new;
mod version;

define_handle!(
    /// Allows interaction and control over GPUs
    pub GpuSubsystem -> GpuSubsystemInner
);
