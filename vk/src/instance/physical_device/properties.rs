use super::PhysicalDevice;
use util::i8_slice_to_cstr;
use vulkan::VkPhysicalDeviceType;

/// The properties of a [`PhysicalDevice`]
pub struct PhysicalDeviceProperties {
    /// The hardware type of the physical device
    pub r#type: VkPhysicalDeviceType,

    /// The name of the physical device
    pub name: String,
}

impl PhysicalDevice {
    /// Gets the properties of this physical device
    pub fn properties(&self) -> PhysicalDeviceProperties {
        let properties = self.f().get_physical_device_properties(self.handle);

        let name = i8_slice_to_cstr(&properties.device_name)
            .to_string_lossy()
            .to_string();

        PhysicalDeviceProperties {
            r#type: properties.device_type,
            name,
        }
    }
}
