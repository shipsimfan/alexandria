use crate::math::{Vector2i, Vector2u};
use vulkan::{VkExtent2D, VkOffset2D};

impl const Into<VkExtent2D> for Vector2u {
    fn into(self) -> VkExtent2D {
        VkExtent2D {
            width: self.x,
            height: self.y,
        }
    }
}

impl const Into<VkOffset2D> for Vector2i {
    fn into(self) -> VkOffset2D {
        VkOffset2D {
            x: self.x,
            y: self.y,
        }
    }
}
