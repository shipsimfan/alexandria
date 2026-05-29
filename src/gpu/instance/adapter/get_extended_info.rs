use crate::gpu::{VulkanAdapter, VulkanExtendedAdapterInfo};

impl<'instance> VulkanAdapter<'instance> {
    /// Gets extended information about this adapter
    pub fn get_extended_info(&self, extended_info: &mut [VulkanExtendedAdapterInfo]) {
        assert!(
            extended_info.len() > 0,
            "at least one extended info structure must be provided"
        );
        match &extended_info[0] {
            VulkanExtendedAdapterInfo::Features(_) => {}
            _ => panic!("the first extended info structure must be a `Features` structure"),
        }

        // Set the next pointers for the extended info structures
        let features = VulkanExtendedAdapterInfo::set_next_chain(extended_info);

        unsafe {
            (self
                .instance
                .functions()
                .adapter
                .get_physical_device_features2)(self.handle, features.cast())
        };
    }
}
