use crate::math::Vector3u;
use vulkan::VkExtent3D;

impl Into<VkExtent3D> for Vector3u {
    fn into(self) -> VkExtent3D {
        VkExtent3D {
            width: self.x,
            height: self.y,
            depth: self.z,
        }
    }
}
