fn main() {
    // Create the Alexandria context with GPU and window support
    let (context, mut pump) = alexandria::AlexandriaContext::<()>::builder()
        .gpu()
        .window()
        .create()
        .unwrap();

    // Create a window
    let window = context
        .window()
        .create_window("Blank Window")
        .create()
        .unwrap();
    println!(
        "Window created at {} with size {}x{}",
        window.position(),
        window.width(),
        window.height()
    );

    // Create the Vulkan instance
    let instance = create_vulkan_instance(&context, &window);
    let debug_messenger = debug_messenger::create(&instance);
    println!(
        "Vulkan instance created with API version {}",
        context.gpu().version().unwrap()
    );

    // Create window surface
    let surface = instance.create_window_surface(&window).unwrap();

    // Run the main event loop
    let mut running = true;
    while running {
        // Wait for the next event and handle it
        let event = pump.wait().unwrap();
        running &= handle_event(&event, &context);

        // Poll for any additional events that may have occurred while handling the previous event
        while let Some(event) = pump.poll().unwrap() {
            running &= handle_event(&event, &context);
        }
    }

    window.destroy().unwrap();
    drop(surface);

    drop(debug_messenger);
    drop(instance);
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
        for layer in gpu.layers().unwrap() {
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
        for extension in gpu.extensions(None).unwrap() {
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
            .unwrap()
    }
}

#[cfg(not(debug_assertions))]
mod debug_messenger {
    /// The required validation layers
    pub const VALIDATION_LAYERS: &[&str] = &[];

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
