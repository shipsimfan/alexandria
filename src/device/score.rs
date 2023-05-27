pub(super) fn calculate_score(properties: &vulkan::PhysicalDeviceProperties) -> usize {
    match properties.device_type {
        vulkan::PhysicalDeviceType::DiscreteGPU => 4000,
        vulkan::PhysicalDeviceType::IntegratedGPU => 3000,
        vulkan::PhysicalDeviceType::VirtualGPU => 2000,
        vulkan::PhysicalDeviceType::CPU => 1000,
        vulkan::PhysicalDeviceType::Other => 0,
    }
}
