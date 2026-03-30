use crate::gpu::{
    VulkanDeviceBuilder, VulkanDeviceExtendedCreateInfo, VulkanDeviceExtension,
    VulkanQueueCreateInfo,
};

impl<'adapter, 'instance, 'a> VulkanDeviceBuilder<'adapter, 'instance, 'a> {
    /// Add a new element of extended information to adjust the device to be created
    pub fn extended_info<E: Into<VulkanDeviceExtendedCreateInfo>>(
        &mut self,
        info: E,
    ) -> &mut VulkanDeviceBuilder<'adapter, 'instance, 'a> {
        self.extended_info.push(info.into());
        self
    }

    /// Add new elements of extended information to adjust the device to be created
    pub fn extended_infos<E: Into<VulkanDeviceExtendedCreateInfo>, I: IntoIterator<Item = E>>(
        &mut self,
        infos: I,
    ) -> &mut VulkanDeviceBuilder<'adapter, 'instance, 'a> {
        self.extended_info.extend(infos.into_iter().map(Into::into));
        self
    }

    /// Add a new queue to be created
    pub fn queue(
        &mut self,
        queue: VulkanQueueCreateInfo<'a>,
    ) -> &mut VulkanDeviceBuilder<'adapter, 'instance, 'a> {
        self.queues.push(queue);
        self
    }

    /// Add new queues to be created
    pub fn queues<I: IntoIterator<Item = VulkanQueueCreateInfo<'a>>>(
        &mut self,
        queues: I,
    ) -> &mut VulkanDeviceBuilder<'adapter, 'instance, 'a> {
        self.queues.extend(queues);
        self
    }

    /// Add a new extension to the list of requested extensions
    pub fn extension(
        &mut self,
        extension: VulkanDeviceExtension,
    ) -> &mut VulkanDeviceBuilder<'adapter, 'instance, 'a> {
        self.extensions.push(extension);
        self
    }

    /// Add new extensions to the list of requested extensions
    pub fn extensions<I: IntoIterator<Item = VulkanDeviceExtension>>(
        &mut self,
        extensions: I,
    ) -> &mut VulkanDeviceBuilder<'adapter, 'instance, 'a> {
        self.extensions.extend(extensions);
        self
    }
}
