use util::{get_device_proc_addr, get_instance_proc_addr};

mod util;

mod device;
mod global;
mod instance;

pub(crate) use device::DeviceFunctions;
pub(crate) use global::GlobalFunctions;
pub(crate) use instance::{
    DebugUtilsFunctions, InstanceFunctions, PhysicalDeviceFunctions, SurfaceFunctions,
    Win32SurfaceFunctions,
};
