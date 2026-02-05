use crate::{
    Result,
    gpu::{GpuSubsystem, subsystem::GpuSubsystemInner},
};

impl GpuSubsystem {
    /// Create a new [`GpuSubsystem`]
    pub(crate) fn new() -> Result<GpuSubsystem> {
        GpuSubsystemInner::new().map(GpuSubsystem::from_inner)
    }
}
