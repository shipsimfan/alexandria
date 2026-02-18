use crate::define_handle;

#[cfg(target_os = "linux")]
use linux::SharedObjectInner;
#[cfg(target_os = "windows")]
use windows::SharedObjectInner;

mod function_symbol;
mod symbol;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

mod get;
mod load_function;
mod load_symbol;
mod open;

pub use function_symbol::FunctionSymbol;
pub use symbol::Symbol;

define_handle!(
    /// A reference to a shared object containing loadable symbols
    pub SharedObject -> SharedObjectInner
);
