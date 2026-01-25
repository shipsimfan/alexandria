mod new;
mod to_vk;

/// The information describing a queue to create
pub struct GraphicsQueueCreateInfo<'a> {
    /// The queue family to create the queues from
    pub queue_family: u32,

    /// The priorities of the queues to create. The count of priorities is the number of queues
    /// that will be created from the queue family.
    ///
    /// Values should be between 0.0 and 1.0.
    pub priorities: &'a [f32],
}
