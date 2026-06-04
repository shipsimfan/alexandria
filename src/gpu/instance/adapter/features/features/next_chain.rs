use crate::gpu::VulkanDeviceFeatures;
use std::ffi::c_void;
use vulkan::{VkStructureType, util::NextChainMut};

impl NextChainMut for VulkanDeviceFeatures {
    fn structure_type(&self) -> VkStructureType {
        self.inner.structure_type()
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        self.inner.as_mut_ptr()
    }

    fn next(&mut self) -> *mut c_void {
        self.inner.next()
    }

    fn set_next(&mut self, next: Option<&mut dyn NextChainMut>) {
        self.inner.set_next(next)
    }
}
