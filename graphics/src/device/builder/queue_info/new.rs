use crate::GraphicsQueueCreateInfo;

impl<'a> GraphicsQueueCreateInfo<'a> {
    /// Create a new [`GraphicsQueueCreateInfo`]
    pub fn new(queue_family: u32, priorities: &'a [f32]) -> GraphicsQueueCreateInfo<'a> {
        GraphicsQueueCreateInfo {
            queue_family,
            priorities,
        }
    }
}
