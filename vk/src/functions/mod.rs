use util::{get_device_proc_addr, get_instance_proc_addr};

mod util;

mod global;
mod instance;

pub(crate) use global::GlobalFunctions;
pub(crate) use instance::{DebugUtilsFunctions, InstanceFunctions, PhysicalDeviceFunctions};
