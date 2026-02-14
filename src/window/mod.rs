//! Subsystem for interacting with platform windowing systems

mod display;
mod subsystem;

pub use display::{Display, DisplayIter};
pub use subsystem::WindowSubsystem;
