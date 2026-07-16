use crate::gpu::VulkanMappedMemory;
use std::ops::{Deref, DerefMut};

impl<T> Deref for VulkanMappedMemory<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr, self.length) }
    }
}

impl<T> DerefMut for VulkanMappedMemory<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.length) }
    }
}
