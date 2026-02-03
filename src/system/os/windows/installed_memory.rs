use crate::MemorySize;
use win32::GetPhysicallyInstalledSystemMemory;

/// Get the total installed memory in the system in bytes
pub fn installed_memory() -> MemorySize {
    let mut memory = 0;
    unsafe { GetPhysicallyInstalledSystemMemory(&mut memory) };
    MemorySize::new(memory * 1024)
}
