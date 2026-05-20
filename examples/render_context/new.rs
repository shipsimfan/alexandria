use crate::{
    RenderContext,
    render_context::{SWAPCHAIN_FORMAT, debug_messenger},
};
use alexandria::{
    AlexandriaContext,
    gpu::{
        VulkanAdapter, VulkanDeviceExtension, VulkanDeviceFeatures, VulkanDeviceVulkan13Features,
        VulkanInstance, VulkanQueueCreateInfo, VulkanSurface, VulkanVersion,
    },
    window::Window,
};

impl RenderContext {
    /// Creates a new [`RenderContext`]
    pub fn new(context: &AlexandriaContext<()>, window: &Window<()>) -> (Self, VulkanSurface) {
        // Create the Vulkan instance
        let instance = create_vulkan_instance(context, window);
        let _debug_messenger = debug_messenger::create(&instance);
        println!(
            "Vulkan instance created with API version {}",
            context.gpu().version().expect("unable to get GPU version")
        );

        // Create window surface
        let surface = instance
            .create_window_surface(&window)
            .expect("unable to create window surface");

        // Select an adapter
        let (adapter, queue_family_index) = find_compatible_adapter(&instance, &surface);

        // Create a device and queue
        let (device, mut queues) = adapter
            .device_builder()
            .extension(VulkanDeviceExtension::Swapchain)
            .queue(VulkanQueueCreateInfo {
                queue_family: queue_family_index,
                priorities: &[1.0],
            })
            .extended_info(VulkanDeviceFeatures::default())
            .extended_info(
                VulkanDeviceVulkan13Features::default()
                    .synchronization2()
                    .dynamic_rendering(),
            )
            .create()
            .unwrap();

        let queue = queues.swap_remove(0);

        // Create command pool and buffer
        let command_pool = device.create_command_pool(queue.queue_family()).unwrap();

        (
            RenderContext {
                _debug_messenger,
                device,
                queue,
                command_pool,
            },
            surface,
        )
    }
}

/// Creates a Vulkan instance with the required extensions for window surface creation
fn create_vulkan_instance(context: &AlexandriaContext<()>, window: &Window<()>) -> VulkanInstance {
    let layers = match debug_messenger::has_layers(context.gpu()) {
        Some(layers) => layers,
        None => {
            println!("Warning: missing validation layers");
            &[]
        }
    };

    let debug_extensions = match debug_messenger::has_extensions(context.gpu()) {
        Some(extensions) => extensions,
        None => {
            println!("Warning: missing debug extensions");
            &[]
        }
    };

    context
        .gpu()
        .instance_builder(VulkanVersion::VERSION_1_0)
        .application(window.title(), VulkanVersion::new(0, 1, 0, 0))
        .engine("Alexandria", VulkanVersion::new(0, 1, 0, 0))
        .layers(layers.into_iter().copied())
        .extensions(debug_extensions.into_iter().copied())
        .window_extensions(&window)
        .api_version(VulkanVersion::VERSION_1_3)
        .create()
        .unwrap()
}

/// Finds a compatible Vulkan adapter for the given surface and returns it along with the index of a compatible queue family
fn find_compatible_adapter<'instance>(
    instance: &'instance VulkanInstance,
    surface: &VulkanSurface,
) -> (VulkanAdapter<'instance>, u32) {
    let adapters = instance
        .enumerate_adapters()
        .expect("unable to get adapters");

    for adapter in adapters {
        for (index, queue_family) in adapter.queue_families().iter().enumerate() {
            if !queue_family.graphics() {
                continue;
            }

            if !adapter
                .supports_surface(index as _, surface)
                .expect("unable to check surface support")
            {
                continue;
            }

            if !adapter
                .swapchain_formats(surface)
                .expect("unable to get swapchain formats")
                .contains(&SWAPCHAIN_FORMAT)
            {
                continue;
            }

            println!("Selected adapter: {}", adapter.name());
            return (adapter, index as u32);
        }
    }

    panic!("no compatible adapter found");
}
