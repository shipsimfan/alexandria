//! Utilities for internal use

mod load_device_function;
mod load_global_function;
mod load_instance_function;

pub(in crate::gpu) use load_device_function::load_device_function;
pub(in crate::gpu) use load_global_function::load_global_function;
pub(in crate::gpu) use load_instance_function::load_instance_function;
