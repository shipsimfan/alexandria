use crate::gpu::{VulkanAdapter, VulkanAdapterFeature};
use vulkan::{VkStructureType, util::create_next_chain_mut};

impl<'instance> VulkanAdapter<'instance> {
    /// Gets the features of this adapter, writing them into the provided feature structures
    pub fn get_features<'a, I: IntoIterator<Item = &'a mut dyn VulkanAdapterFeature>>(
        &self,
        features: I,
    ) {
        let mut features = features.into_iter().peekable();
        let first = features
            .peek()
            .expect("at least one feature structure must be provided to get the adapter features");
        assert_eq!(
            first.structure_type(),
            VkStructureType::PhysicalDeviceFeatures2,
            "the first feature structure must be of type `VulkanDeviceFeatures`"
        );

        // Set the next pointers for the extended info structures
        let features = create_next_chain_mut(features.map(|feature| feature as _));

        unsafe {
            (self
                .instance
                .functions()
                .adapter
                .get_physical_device_features2)(self.handle, features.cast())
        };
    }
}
