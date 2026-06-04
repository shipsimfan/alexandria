use crate::{
    RenderContext,
    render_context::{SWAPCHAIN_COLOR_SPACE, SWAPCHAIN_FORMAT, debug_messenger},
};
use alexandria::{
    AlexandriaContext,
    gpu::{
        VulkanAdapter, VulkanCommandPoolCreateFlag, VulkanDeviceExtension, VulkanDeviceFeatures,
        VulkanDeviceVulkan13Features, VulkanInstance, VulkanQueueCreateInfo, VulkanSurface,
        VulkanSurfaceFormat, VulkanVersion,
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
            .queue(VulkanQueueCreateInfo::new(queue_family_index, &[1.0]))
            .feature(
                &mut VulkanDeviceVulkan13Features::default()
                    .enable_synchronization2()
                    .enable_dynamic_rendering(),
            )
            .create()
            .unwrap();

        let queue = queues.swap_remove(0);

        // Create command pool and buffer
        let command_pool = device
            .create_command_pool(
                queue.queue_family(),
                VulkanCommandPoolCreateFlag::ResetCommandBuffer,
            )
            .unwrap();

        (
            RenderContext {
                _debug_messenger,
                device,
                queue,
                command_pool,
                command_buffers: Vec::new(),
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
        .instance_builder(VulkanVersion::VERSION_1_3)
        .application(window.title(), VulkanVersion::new(0, 1, 0, 0))
        .engine("Alexandria", VulkanVersion::new(0, 1, 0, 0))
        .layers(layers.into_iter().copied())
        .extensions(debug_extensions.into_iter().copied())
        .window_extensions(&window)
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
                .contains(&VulkanSurfaceFormat {
                    format: SWAPCHAIN_FORMAT,
                    color_space: SWAPCHAIN_COLOR_SPACE,
                })
            {
                continue;
            }

            let mut features = VulkanDeviceFeatures::default();
            let mut vulkan_13_features = VulkanDeviceVulkan13Features::default();
            adapter.get_features([&mut features as &mut _, &mut vulkan_13_features as _]);
            if !vulkan_13_features.synchronization2() || !vulkan_13_features.dynamic_rendering() {
                continue;
            }

            println!("Selected adapter: {}", adapter.name());
            return (adapter, index as u32);
        }
    }

    panic!("no compatible adapter found");
}
