use crate::math::{Vector2i, Vector2u};
use vulkan::{VkExtent2D, VkOffset2D};

impl const From<VkExtent2D> for Vector2u {
    fn from(value: VkExtent2D) -> Self {
        Vector2u::new(value.width, value.height)
    }
}

impl const From<VkOffset2D> for Vector2i {
    fn from(value: VkOffset2D) -> Self {
        Vector2i::new(value.x, value.y)
    }
}
