use win32::{GetSystemInfo, SYSTEM_INFO};

/// Get the number of logical CPU cores on this system
pub fn cpu_cores() -> u32 {
    let mut system_info = SYSTEM_INFO::default();
    unsafe { GetSystemInfo(&mut system_info) };

    system_info.number_of_processors
}
