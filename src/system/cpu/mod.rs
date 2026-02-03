mod architecture;

#[cfg(target_arch = "x86_64")]
mod x86_64;

pub use architecture::Architecture;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
