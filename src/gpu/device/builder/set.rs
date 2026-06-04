use crate::gpu::{
    VulkanAdapterFeature, VulkanDeviceBuilder, VulkanDeviceExtension, VulkanQueueCreateInfo,
};

impl<'adapter, 'instance, 'a> VulkanDeviceBuilder<'adapter, 'instance, 'a> {
    /// Add a new feature to adjust the device to be created
    pub fn feature(
        &mut self,
        info: &'a mut dyn VulkanAdapterFeature,
    ) -> &mut VulkanDeviceBuilder<'adapter, 'instance, 'a> {
        self.features.push(info);
        self
    }

    /// Add new features to adjust the device to be created
    pub fn features<I: IntoIterator<Item = &'a mut dyn VulkanAdapterFeature>>(
        &mut self,
        infos: I,
    ) -> &mut VulkanDeviceBuilder<'adapter, 'instance, 'a> {
        self.features.extend(infos);
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
