use crate::define_handle;
use inner::VulkanImageInner;

mod inner;

mod create_image_view;
mod get;
mod new;

define_handle!(
    /// An image on the GPU
    pub VulkanImage -> VulkanImageInner
);
