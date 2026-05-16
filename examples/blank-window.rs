const SWAPCHAIN_FORMAT: alexandria::gpu::VulkanFormat =
    alexandria::gpu::VulkanFormat::B8G8R8A8UNorm;

const SWAPCHAIN_PRESENT_MODE: alexandria::gpu::VulkanSwapchainPresentMode =
    alexandria::gpu::VulkanSwapchainPresentMode::Fifo;

const MAX_FRAMES_IN_FLIGHT: usize = 2;

struct FrameData {
    command_buffer: alexandria::gpu::VulkanCommandBuffer,
    render_complete_semaphore: alexandria::gpu::VulkanSemaphore,
    present_complete_semaphore: alexandria::gpu::VulkanSemaphore,
    draw_fence: alexandria::gpu::VulkanFence,
}

impl FrameData {
    pub fn new(
        graphics_device: &alexandria::gpu::VulkanDevice,
        command_pool: &alexandria::gpu::VulkanCommandPool,
    ) -> Self {
        let command_buffer = command_pool.allocate_command_buffer().unwrap();
        let render_complete_semaphore = graphics_device.create_semaphore().unwrap();
        let present_complete_semaphore = graphics_device.create_semaphore().unwrap();
        let draw_fence = graphics_device.create_fence(true).unwrap();

        FrameData {
            command_buffer,
            render_complete_semaphore,
            present_complete_semaphore,
            draw_fence,
        }
    }
}

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
        .resizable()
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
        .extended_info(alexandria::gpu::VulkanDeviceFeatures::default())
        .extended_info(
            alexandria::gpu::VulkanDeviceVulkan13Features::default()
                .synchronization2()
                .dynamic_rendering(),
        )
        .create()
        .unwrap();

    let mut queue = queues.swap_remove(0);

    // Create swapchain
    let (mut swapchain, mut image_views) = create_swapchain(&graphics_device, &surface, &window);
    let swapchain_size = window.size();

    // Create command pool and buffer
    let command_pool = graphics_device
        .create_command_pool(queue.queue_family())
        .unwrap();

    // Create per-frame data
    let mut frames = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);
    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        frames.push(FrameData::new(&graphics_device, &command_pool));
    }

    // Run the main event loop
    let mut running = true;
    let mut frame_index = 0;
    let mut should_recreate_swapchain = false;
    while running {
        let frame = &mut frames[frame_index];

        // Wait for the previous frame to finish
        frame.draw_fence.wait(u64::MAX).unwrap();

        // Get the next image from the swapchain
        should_recreate_swapchain |= if window.is_minimized() {
            // Wait for the next event and handle it
            let event = pump.wait().expect("unable to wait for event");
            running &= handle_event(&event, &context);

            true
        } else {
            match swapchain
                .acquire_next_image(u64::MAX, &frame.present_complete_semaphore)
                .unwrap()
            {
                Some(image_index) => {
                    // Draw a blank frame
                    render_frame(
                        frame,
                        image_index,
                        &swapchain,
                        &image_views,
                        &mut queue,
                        &window,
                    );

                    frame_index = (frame_index + 1) % MAX_FRAMES_IN_FLIGHT;

                    // Wait for the next event and handle it
                    let event = pump.wait().expect("unable to wait for event");
                    running &= handle_event(&event, &context);

                    false
                }
                None => true,
            }
        };

        // Poll for any additional events that may have occurred while handling the previous event
        while let Some(event) = pump.poll().expect("unable to poll for event") {
            running &= handle_event(&event, &context);
        }

        if (should_recreate_swapchain || window.size() != swapchain_size) && !window.is_minimized()
        {
            should_recreate_swapchain = false;
            graphics_device.wait_idle().unwrap();

            drop(image_views);
            drop(swapchain);
            (swapchain, image_views) = create_swapchain(&graphics_device, &surface, &window);

            for frame in &mut frames {
                frame.present_complete_semaphore = graphics_device.create_semaphore().unwrap();
                frame.render_complete_semaphore = graphics_device.create_semaphore().unwrap();
            }
        }
    }

    graphics_device.wait_idle().unwrap();

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
        .api_version(alexandria::gpu::VulkanVersion::VERSION_1_3)
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

fn create_swapchain<'surface>(
    graphics_device: &alexandria::gpu::VulkanDevice,
    surface: &'surface alexandria::gpu::VulkanSurface,
    window: &alexandria::window::Window<()>,
) -> (
    alexandria::gpu::VulkanSwapchain<'surface>,
    Vec<alexandria::gpu::VulkanImageView>,
) {
    let swapchain = graphics_device
        .create_swapchain(
            3,
            SWAPCHAIN_FORMAT,
            window.size(),
            SWAPCHAIN_PRESENT_MODE,
            &surface,
        )
        .unwrap();
    let mut image_views = Vec::with_capacity(swapchain.images().len());
    for image in swapchain.images() {
        image_views.push(image.create_image_view(SWAPCHAIN_FORMAT).unwrap());
    }

    (swapchain, image_views)
}

fn render_frame(
    frame: &mut FrameData,
    image_index: usize,
    swapchain: &alexandria::gpu::VulkanSwapchain,
    image_views: &[alexandria::gpu::VulkanImageView],
    queue: &mut alexandria::gpu::VulkanQueue,
    window: &alexandria::window::Window<()>,
) {
    frame.draw_fence.reset().unwrap();

    frame.command_buffer.begin().unwrap();

    frame.command_buffer.cmd_pipeline_barrier2(
        &swapchain.images()[image_index],
        alexandria::gpu::VulkanImageLayout::Undefined,
        alexandria::gpu::VulkanImageLayout::ColorAttachmentOptimal,
        alexandria::gpu::VulkanAccessFlags::default(),
        alexandria::gpu::VulkanAccessFlag::ColorAttachmentWrite,
        alexandria::gpu::VulkanPipelineStageFlag::ColorAttachmentOutput,
        alexandria::gpu::VulkanPipelineStageFlag::ColorAttachmentOutput,
    );

    let clear_color = alexandria::math::Color4f::<alexandria::math::Srgb>::new(1.0, 0.0, 1.0, 1.0);

    frame
        .command_buffer
        .cmd_begin_rendering(&image_views[image_index], window.size(), clear_color);
    frame.command_buffer.cmd_end_rendering();

    frame.command_buffer.cmd_pipeline_barrier2(
        &swapchain.images()[image_index],
        alexandria::gpu::VulkanImageLayout::ColorAttachmentOptimal,
        alexandria::gpu::VulkanImageLayout::PresentSrcKhr,
        alexandria::gpu::VulkanAccessFlag::ColorAttachmentWrite,
        alexandria::gpu::VulkanAccessFlags::default(),
        alexandria::gpu::VulkanPipelineStageFlag::ColorAttachmentOutput,
        alexandria::gpu::VulkanPipelineStageFlag::BottomOfPipe,
    );

    frame.command_buffer.end().unwrap();

    queue
        .submit(
            &frame.command_buffer,
            &frame.present_complete_semaphore,
            &frame.render_complete_semaphore,
            &mut frame.draw_fence,
        )
        .unwrap();

    queue
        .present(
            &&frame.render_complete_semaphore,
            &swapchain,
            image_index as _,
        )
        .unwrap();
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
