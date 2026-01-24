use crate::GraphicsQueueCreateInfo;

impl<'a> GraphicsQueueCreateInfo<'a> {
    /// Create a new [`GraphicsQueueCreateInfo`]
    pub fn new(queue_family: u32, priorities: &'a [f32]) -> GraphicsQueueCreateInfo<'a> {
        assert!(priorities.len() > 0);

        GraphicsQueueCreateInfo {
            queue_family,
            priorities,
        }
    }
}
