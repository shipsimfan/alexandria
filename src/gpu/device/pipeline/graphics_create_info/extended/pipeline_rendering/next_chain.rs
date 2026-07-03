use crate::gpu::VulkanPipelineRenderingCreateInfo;
use std::ffi::c_void;
use vulkan::{VkStructureType, util::NextChain};

impl<'a> NextChain for VulkanPipelineRenderingCreateInfo<'a> {
    fn structure_type(&self) -> VkStructureType {
        self.inner.structure_type()
    }

    fn as_ptr(&self) -> *const c_void {
        self.inner.as_ptr()
    }

    fn next(&self) -> *const c_void {
        self.inner.next()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.inner.set_next(next)
    }
}
