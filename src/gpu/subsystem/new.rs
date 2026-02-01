use crate::{Result, gpu::GpuSubsystem};
use std::marker::PhantomData;

impl GpuSubsystem {
    /// Create a new [`GpuSubsystem`]
    pub(crate) fn new() -> Result<GpuSubsystem> {
        Ok(GpuSubsystem { _priv: PhantomData })
    }
}
