mod drop;
mod get;
mod new;

/// A region that has been memory-mapped
pub(in crate::window::subsystem::linux::wayland::seat_listener) struct MMapRegion {
    /// The pointer to the start of the memory region
    ptr: *mut u8,

    /// The size of the memory region, in bytes
    size: usize,
}
