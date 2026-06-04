use crate::{
    Error, Result,
    gpu::{VulkanFence, VulkanQueue, VulkanSubmitInfo},
};
use vulkan::{VkFence, try_vulkan};

impl VulkanQueue {
    /// Submits command buffers to the queue
    pub fn submit(
        &mut self,
        submit_infos: &[VulkanSubmitInfo],
        fence: Option<&mut VulkanFence>,
    ) -> Result<()> {
        try_vulkan!((self.device.functions().queue.submit2)(
            self.handle,
            submit_infos.len() as _,
            submit_infos.as_ptr().cast(),
            fence.map_or(VkFence::null(), |f| f.handle())
        ))
        .map_err(|vk| Error::new_with("unable to submit queue", vk))?;

        Ok(())
    }
}
