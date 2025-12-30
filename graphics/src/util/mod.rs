//! Utilities for internal use

mod load_global_function;
mod load_instance_function;

pub(crate) use load_global_function::load_global_function;
pub(crate) use load_instance_function::load_instance_function;
