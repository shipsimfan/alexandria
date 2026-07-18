use crate::math::Vector3u;
use vulkan::VkExtent3D;

const impl From<VkExtent3D> for Vector3u {
    fn from(value: VkExtent3D) -> Self {
        Vector3u::new(value.width, value.height, value.depth)
    }
}
