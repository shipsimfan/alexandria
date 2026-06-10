use crate::{Error, Result, window::subsystem::linux::wayland::seat_listener::MMapRegion};
use linux::sys::mman::{MAP_FAILED, MAP_PRIVATE, PROT_READ, mmap};
use std::ptr::null_mut;

impl MMapRegion {
    /// Create a new [`MMapRegion`] from the given file descriptor and size
    pub fn new(fd: i32, size: usize) -> Result<MMapRegion> {
        let ptr = unsafe { mmap(null_mut(), size, PROT_READ, MAP_PRIVATE, fd, 0) };
        if ptr == MAP_FAILED {
            return Err(Error::new_with(
                "unable to memory map a region",
                linux::Error::errno(),
            ));
        }

        Ok(MMapRegion {
            ptr: ptr.cast(),
            size,
        })
    }
}
