use std::marker::PhantomData;

mod new;

/// Allows interaction and control over GPUs
pub struct GpuSubsystem {
    /// A value to prevent this from being made externally
    _priv: PhantomData<()>,
}
