mod create;
mod new;
mod set;

/// A builder for an [`AlexandriaContext`](crate::AlexandriaContext)
pub struct AlexandriaContextBuilder {
    /// Should the GPU subsystem be initialized?
    gpu: bool,

    /// Should the window subsystem be initialized? This implies `gpu` too
    window: bool,
}
