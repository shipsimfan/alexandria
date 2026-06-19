use crate::gpu::VulkanViewport;

impl VulkanViewport {
    /// Get the x coordinate of the viewport
    pub fn x(&self) -> f32 {
        self.inner.x
    }

    /// Get the y coordinate of the viewport
    pub fn y(&self) -> f32 {
        self.inner.y
    }

    /// Get the width of the viewport
    pub fn width(&self) -> f32 {
        self.inner.width
    }

    /// Get the height of the viewport
    pub fn height(&self) -> f32 {
        self.inner.height
    }

    /// Get the minimum depth of the viewport
    pub fn min_depth(&self) -> f32 {
        self.inner.min_depth
    }

    /// Get the maximum depth of the viewport
    pub fn max_depth(&self) -> f32 {
        self.inner.max_depth
    }
}
