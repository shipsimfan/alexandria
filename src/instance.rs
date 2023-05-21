use crate::Result;
use vulkan::{VkApplicationInfo, VkInstance, Vulkan, VK_API_VERSION_1_0};

pub struct Instance {
    instance: VkInstance,
}

impl Instance {
    pub fn new(app_name: &str, app_version: u32) -> Result<Self> {
        assert_eq!(app_name.as_bytes()[app_name.len() - 1], 0);

        let vulkan = Vulkan::new_native()?;

        let instance = vulkan.create_instance(&vulkan::VkInstanceCreateInfo::new(
            vulkan::VkInstanceCreateFlags::new(&[]),
            Some(&VkApplicationInfo::new(
                Some(app_name),
                app_version,
                Some("Alexandria\0"),
                0,
                VK_API_VERSION_1_0,
            )),
            &[],
            crate::os::get_extension_list(),
        ))?;

        Ok(Instance { instance })
    }
}
