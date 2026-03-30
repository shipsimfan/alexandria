const SWAPCHAIN_FORMAT: alexandria::gpu::SwapchainFormat =
    alexandria::gpu::SwapchainFormat::B8G8R8A8Srgb;

const SWAPCHAIN_PRESENT_MODE: alexandria::gpu::SwapchainPresentMode =
    alexandria::gpu::SwapchainPresentMode::Fifo;

fn main() {
    // Create the Alexandria context with GPU and window support
    let (context, mut pump) = alexandria::AlexandriaContext::<()>::builder()
        .gpu()
        .window()
        .create()
        .expect("unable to create Alexandria context");

    // Create a window
    let window = context
        .window()
        .create_window("Blank Window")
        .create()
        .expect("unable to create window");
    println!(
        "Window created at {} with size {}x{}",
        window.position(),
        window.width(),
        window.height()
    );

    // Create the Vulkan instance
    let instance = create_vulkan_instance(&context, &window);
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
    let (graphics_device, mut queues) = adapter
        .device_builder()
        .extension(alexandria::gpu::VulkanDeviceExtension::Swapchain)
        .queue(alexandria::gpu::VulkanQueueCreateInfo {
            queue_family: queue_family_index,
            priorities: &[1.0],
        })
        .create()
        .unwrap();

    let queue = queues.swap_remove(0);

    // Run the main event loop
    let mut running = true;
    while running {
        // Wait for the next event and handle it
        let event = pump.wait().expect("unable to wait for event");
        running &= handle_event(&event, &context);

        // Poll for any additional events that may have occurred while handling the previous event
        while let Some(event) = pump.poll().expect("unable to poll for event") {
            running &= handle_event(&event, &context);
        }
    }

    window.destroy().expect("unable to destroy window");
}

/// Handles an event and returns whether the application should continue running
fn handle_event(event: &alexandria::Event<()>, _: &alexandria::AlexandriaContext<()>) -> bool {
    match event.kind {
        alexandria::EventKind::Quit | alexandria::EventKind::WindowCloseRequest { .. } => false,
        _ => true,
    }
}

/// Creates a Vulkan instance with the required extensions for window surface creation
fn create_vulkan_instance(
    context: &alexandria::AlexandriaContext<()>,
    window: &alexandria::window::Window<()>,
) -> alexandria::gpu::VulkanInstance {
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
        .instance_builder(alexandria::gpu::VulkanVersion::VERSION_1_0)
        .application(
            "Blank Window Example",
            alexandria::gpu::VulkanVersion::new(0, 1, 0, 0),
        )
        .engine(
            "Alexandria",
            alexandria::gpu::VulkanVersion::new(0, 1, 0, 0),
        )
        .layers(layers.into_iter().copied())
        .extensions(debug_extensions.into_iter().copied())
        .window_extensions(&window)
        .create()
        .unwrap()
}

/// Finds a compatible Vulkan adapter for the given surface and returns it along with the index of a compatible queue family
fn find_compatible_adapter<'instance>(
    instance: &'instance alexandria::gpu::VulkanInstance,
    surface: &alexandria::gpu::VulkanSurface,
) -> (alexandria::gpu::VulkanAdapter<'instance>, u32) {
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

#[cfg(debug_assertions)]
mod debug_messenger {
    pub struct DebugCallback;

    impl alexandria::gpu::VulkanDebugMessengerCallback for DebugCallback {
        fn message(&self, message: &str, severity: alexandria::gpu::VulkanDebugMessageSeverity) {
            println!("[{}] {}", severity, message);
        }
    }

    /// Does this system have required Vulkan validation layers?
    pub fn has_layers(gpu: &alexandria::gpu::GpuSubsystem) -> Option<&'static [&'static str]> {
        for layer in gpu.layers().expect("unable to get layers") {
            if layer.name() == "VK_LAYER_KHRONOS_validation" {
                return Some(&["VK_LAYER_KHRONOS_validation"]);
            }
        }

        None
    }

    /// Does this system have required Vulkan validation extensions?
    pub fn has_extensions(
        gpu: &alexandria::gpu::GpuSubsystem,
    ) -> Option<&'static [alexandria::gpu::VulkanInstanceExtension]> {
        for extension in gpu.extensions(None).expect("unable to get extensions") {
            if extension == alexandria::gpu::VulkanInstanceExtension::DebugUtils {
                return Some(&[alexandria::gpu::VulkanInstanceExtension::DebugUtils]);
            }
        }

        None
    }

    /// Create a new debug messenger
    pub fn create(
        instance: &alexandria::gpu::VulkanInstance,
    ) -> alexandria::gpu::VulkanDebugMessenger<DebugCallback> {
        instance
            .create_debug_messenger(
                alexandria::gpu::VulkanDebugMessageSeverity::Verbose,
                DebugCallback,
            )
            .expect("unable to create debug messenger")
    }
}

#[cfg(not(debug_assertions))]
mod debug_messenger {
    /// Does this system have required Vulkan validation layers?
    pub fn has_layers(_: &alexandria::gpu::GpuSubsystem) -> Option<&'static [&'static str]> {
        Some(&[])
    }

    /// Does this system have required Vulkan validation extensions?
    pub fn has_extensions(
        _: &alexandria::gpu::GpuSubsystem,
    ) -> Option<&'static [alexandria::gpu::VulkanInstanceExtension]> {
        Some(&[])
    }

    /// Create a new debug messenger
    pub fn create(_: &alexandria::gpu::VulkanInstance) -> () {
        ()
    }
}
