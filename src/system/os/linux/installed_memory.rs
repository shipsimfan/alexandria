use crate::MemorySize;
use linux::unistd::{_SC_PAGESIZE, _SC_PHYS_PAGES, sysconf};

/// Get the total installed memory in the system in bytes
pub fn installed_memory() -> MemorySize {
    let page_size = unsafe { sysconf(_SC_PAGESIZE) } as u64;
    let pages = unsafe { sysconf(_SC_PHYS_PAGES) } as u64;
    MemorySize::new(page_size * pages)
}
