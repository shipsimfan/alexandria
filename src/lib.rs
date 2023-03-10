#[cfg(target_os = "windows")]
use dx12 as system;

pub use system::*;

pub use common::{DebugMessage, DebugMessageLevel, MessageBoxClass};
