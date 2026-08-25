use crate::math::{Vector3i, Vector3u};
use vulkan::{VkExtent3D, VkOffset3D};

const impl Into<VkExtent3D> for Vector3u {
    fn into(self) -> VkExtent3D {
        VkExtent3D {
            width: self.x,
            height: self.y,
            depth: self.z,
        }
    }
}

const impl Into<VkOffset3D> for Vector3i {
    fn into(self) -> VkOffset3D {
        VkOffset3D {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
