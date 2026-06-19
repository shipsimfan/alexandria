use crate::gpu::VulkanViewport;

impl VulkanViewport {
    /// Set the x coordinate of the viewport
    pub fn set_x(mut self, x: f32) -> Self {
        self.inner.x = x;
        self
    }

    /// Set the y coordinate of the viewport
    pub fn set_y(mut self, y: f32) -> Self {
        self.inner.y = y;
        self
    }

    /// Set the width of the viewport
    pub fn set_width(mut self, width: f32) -> Self {
        self.inner.width = width;
        self
    }

    /// Set the height of the viewport
    pub fn set_height(mut self, height: f32) -> Self {
        self.inner.height = height;
        self
    }

    /// Set the minimum depth of the viewport
    pub fn set_min_depth(mut self, min_depth: f32) -> Self {
        self.inner.min_depth = min_depth;
        self
    }

    /// Set the maximum depth of the viewport
    pub fn set_max_depth(mut self, max_depth: f32) -> Self {
        self.inner.max_depth = max_depth;
        self
    }
}
