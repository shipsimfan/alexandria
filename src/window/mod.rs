//! Subsystem for interacting with platform windowing systems

mod display;
mod subsystem;

pub use display::{Display, DisplayIter, DisplayMode, DisplayOrientation};
pub use subsystem::WindowSubsystem;
