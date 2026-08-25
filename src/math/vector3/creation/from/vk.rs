use crate::math::{Vector3i, Vector3u};
use vulkan::{VkExtent3D, VkOffset3D};

const impl From<VkExtent3D> for Vector3u {
    fn from(value: VkExtent3D) -> Self {
        Vector3u::new(value.width, value.height, value.depth)
    }
}

const impl From<VkOffset3D> for Vector3i {
    fn from(value: VkOffset3D) -> Self {
        Vector3i::new(value.x, value.y, value.z)
    }
}
