use crate::gpu::{VulkanDeviceExtendedDynamicStateFeatures, VulkanExtendedAdapterInfo};
use std::ptr::null_mut;

impl VulkanDeviceExtendedDynamicStateFeatures {
    /// Sets the `next` pointer of this extended adapter info to point to `next`
    pub(in crate::gpu::instance::adapter::extended_info) fn set_next(
        &mut self,
        next: Option<&mut VulkanExtendedAdapterInfo>,
    ) {
        self.inner.next = next.map_or(null_mut(), VulkanExtendedAdapterInfo::as_mut_ptr);
    }
}
