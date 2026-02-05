use crate::define_handle;
use functions::GlobalVulkanFunctions;
use inner::GpuSubsystemInner;

mod functions;
mod inner;

mod instance_builder;
mod new;

define_handle!(
    /// Allows interaction and control over GPUs
    pub GpuSubsystem -> GpuSubsystemInner
);
