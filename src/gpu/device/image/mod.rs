use crate::define_handle;
use inner::VulkanImageInner;

mod functions;
mod inner;

mod create_image_view;
mod get;
mod get_memory_requirements;
mod new;

pub(in crate::gpu::device) use functions::*;

define_handle!(
    /// An image on the GPU
    pub VulkanImage -> VulkanImageInner
);
