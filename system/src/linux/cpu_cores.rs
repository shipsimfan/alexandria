use linux::sys::sysinfo::get_nprocs;

/// Get the number of logical CPU cores on this system
pub fn cpu_cores() -> u32 {
    unsafe { get_nprocs() as _ }
}
