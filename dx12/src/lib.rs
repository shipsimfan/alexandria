#![feature(negative_impls)]

mod adapter;
mod display;
mod error;
mod instance;
mod message_box;
mod window;

pub use adapter::{Adapter, DisplayIter};
pub use display::{Display, RefreshRate, Resolution};
pub use error::{Error, ErrorKind};
pub use instance::{AdapterIter, Instance};
pub use message_box::MessageBox;
pub use window::{Graphics3D, Window};

pub use common::{DebugMessage, DebugMessageLevel, MessageBoxClass};

pub type Result<T> = core::result::Result<T, Error>;

const DISPLAY_FORMAT: win32::DXGIFormat = win32::DXGIFormat::R8G8B8A8Unorm;
