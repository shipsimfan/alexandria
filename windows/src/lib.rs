#![feature(negative_impls)]
#![feature(new_uninit)]

mod extensions;
mod instance;
mod window;

pub use extensions::get_extension_list;
pub use instance::Instance;
pub use window::Window;

pub use win32::{Error, Result};
