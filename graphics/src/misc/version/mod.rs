mod constants;
mod display;
mod enumerate;
mod get;
mod new;
mod supports;

/// The version of a [`GraphicsInstance`](crate::GraphicsInstance)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphicsVersion {
    /// The version of the instance, encoded
    version: u32,
}
