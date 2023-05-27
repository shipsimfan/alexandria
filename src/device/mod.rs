use crate::{Instance, Result};
use score::calculate_score;
use std::sync::Arc;
use swapchain::SwapchainSettings;

mod compatibility;
mod queue_families;
mod score;
mod swapchain;

pub use vulkan::VK_UUID_SIZE as DEVICE_UUID_SIZE;

pub(crate) use queue_families::QueueFamilyIndices;

pub struct Device {
    inner: vulkan::PhysicalDevice,
    instance: Arc<Instance>,

    id: (u32, u32),
    score: usize,
    name: String,
    uuid: [u8; DEVICE_UUID_SIZE],

    queue_family_indices: QueueFamilyIndices,
    swapchain_settings: SwapchainSettings,
}

impl Device {
    pub(crate) fn new(
        inner: vulkan::PhysicalDevice,
        instance: Arc<Instance>,
        surface: &vulkan::Surface,
    ) -> Option<Result<Self>> {
        let properties = inner.get_properties();
        let queue_family_indices = QueueFamilyIndices::get(&inner);

        if match compatibility::is_compatible(&inner, &queue_family_indices, surface) {
            Ok(compatible) => compatible,
            Err(error) => return Some(Err(error)),
        } {
            let swapchain_settings = match SwapchainSettings::get(surface, &inner) {
                Ok(swapchain_settings) => match swapchain_settings {
                    Some(swapchain_settings) => swapchain_settings,
                    None => return None,
                },
                Err(error) => return Some(Err(error)),
            };

            let score = calculate_score(&properties);

            Some(Ok(Device {
                inner,
                instance,

                id: (properties.vendor_id, properties.device_id),
                score,
                name: properties.device_name,
                uuid: properties.pipelane_chache_uuid,

                queue_family_indices,
                swapchain_settings,
            }))
        } else {
            None
        }
    }

    pub fn id(&self) -> (u32, u32) {
        self.id
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> &[u8; DEVICE_UUID_SIZE] {
        &self.uuid
    }

    pub(crate) fn consume(
        self,
    ) -> (
        vulkan::PhysicalDevice,
        Arc<Instance>,
        QueueFamilyIndices,
        SwapchainSettings,
    ) {
        (
            self.inner,
            self.instance,
            self.queue_family_indices,
            self.swapchain_settings,
        )
    }
}
