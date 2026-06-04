use crate::{
    Id,
    gpu::{VulkanCommandBuffer, VulkanCommandPool},
};
use std::ops::{Index, IndexMut};

impl Index<Id<VulkanCommandBuffer>> for VulkanCommandPool {
    type Output = VulkanCommandBuffer;

    fn index(&self, id: Id<VulkanCommandBuffer>) -> &Self::Output {
        self.get(id).expect("invalid command buffer ID")
    }
}

impl IndexMut<Id<VulkanCommandBuffer>> for VulkanCommandPool {
    fn index_mut(&mut self, id: Id<VulkanCommandBuffer>) -> &mut Self::Output {
        self.get_mut(id).expect("invalid command buffer ID")
    }
}
