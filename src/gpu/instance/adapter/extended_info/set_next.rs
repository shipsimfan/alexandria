use crate::gpu::VulkanExtendedAdapterInfo;
use std::{ffi::c_void, ptr::null_mut};

impl VulkanExtendedAdapterInfo {
    /// Sets the `next` pointer of a chain of extended adapter info structures to point to the next
    /// structure in the slice, with the final one pointing to `None`, returning a pointer to the
    /// first structure in the slice, or `null_mut` if the slice is empty
    pub(in crate::gpu) fn set_next_chain(
        extended_info: &mut [VulkanExtendedAdapterInfo],
    ) -> *mut c_void {
        if extended_info.len() == 0 {
            return null_mut();
        }

        for i in 0..extended_info.len() {
            if i < extended_info.len() - 1 {
                let (left, right) = extended_info.split_at_mut(i + 1);
                left[i].set_next(Some(&mut right[0]));
            } else {
                extended_info[i].set_next(None);
            }
        }

        extended_info[0].as_mut_ptr()
    }

    /// Sets the `next` pointer of this extended adapter info to point to `next`
    pub(in crate::gpu) fn set_next(&mut self, next: Option<&mut VulkanExtendedAdapterInfo>) {
        match self {
            VulkanExtendedAdapterInfo::Features(features) => features.set_next(next),
            VulkanExtendedAdapterInfo::Vulkan13Features(features) => features.set_next(next),
            VulkanExtendedAdapterInfo::ExtendedDynamicStateFeatures(features) => {
                features.set_next(next)
            }
        }
    }
}
