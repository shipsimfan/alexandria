const TITLE: &str = "Blank Window Example";

fn main() {
    // Create window
    let mut window = alexandria_graphics::window::Window::builder(TITLE)
        .build()
        .unwrap();

    // Create Vulkan instance
    let instance = create_instance(&window);
    let debug_messenger = debug_messenger::create(&instance);

    // Create window surface
    let surface = instance.create_window_surface(&mut window).unwrap();

    // Select adapter
    let (adapter, (queue_family_index, swapchain_format)) = instance
        .enumerate_adapters()
        .unwrap()
        .into_iter()
        .filter_map(|adapter| {
            is_adapter_supported(&adapter, &surface).map(|index| (adapter, index))
        })
        .next()
        .expect("no compatible adapters");

    println!(
        "Using adapter \"{}\" with queue family {} and swapchain format {:?}",
        adapter.name(),
        queue_family_index,
        swapchain_format
    );

    // Main loop
    while !window.is_close_requested() {
        window.process_messages().unwrap();
    }

    drop(surface);
    drop(debug_messenger);
}

/// Checks if `surface` is supported by `adapter`, returning the lowest queue family index that supports presenting and graphics
fn is_adapter_supported(
    adapter: &alexandria_graphics::GraphicsAdapter,
    surface: &alexandria_graphics::WindowSurface,
) -> Option<(u32, alexandria_graphics::SwapchainFormat)> {
    for queue_family in adapter.queue_families() {
        if !queue_family.graphics() {
            continue;
        }

        if !adapter
            .supports_surface(queue_family.index(), surface)
            .unwrap()
        {
            continue;
        }

        let mut formats = adapter.swapchain_formats(surface).unwrap();
        if formats.len() == 0 {
            continue;
        }

        let format = if formats.contains(&alexandria_graphics::SwapchainFormat::B8G8R8A8Srgb) {
            alexandria_graphics::SwapchainFormat::B8G8R8A8Srgb
        } else if formats.contains(&alexandria_graphics::SwapchainFormat::R8G8B8A8Srgb) {
            alexandria_graphics::SwapchainFormat::R8G8B8A8Srgb
        } else {
            formats.swap_remove(0)
        };

        return Some((queue_family.index(), format));
    }

    None
}

fn create_instance(
    window: &alexandria_graphics::window::Window,
) -> alexandria_graphics::GraphicsInstance {
    let layers = match debug_messenger::has_layers() {
        Some(layers) => layers,
        None => {
            println!("Warning: missing validation layers");
            &[]
        }
    };

    let debug_extensions = match debug_messenger::has_extensions() {
        Some(extensions) => extensions,
        None => {
            println!("Warning: missing debug extensions");
            &[]
        }
    };

    alexandria_graphics::GraphicsInstance::builder(
        alexandria_graphics::GraphicsVersion::VERSION_1_4,
    )
    .application(TITLE, alexandria_graphics::GraphicsVersion::VERSION_1_0)
    .layers(layers.into_iter().map(|l| *l))
    .extensions(debug_extensions.into_iter().map(|e| *e))
    .window_extensions(&window)
    .create()
    .unwrap()
}

#[cfg(debug_assertions)]
mod debug_messenger {
    pub struct DebugCallback;

    impl alexandria_graphics::GraphicsDebugMessengerCallback for DebugCallback {
        fn message(
            &self,
            message: &str,
            severity: alexandria_graphics::GraphicsDebugMessageSeverity,
        ) {
            println!("[{}] {}", severity, message);
        }
    }

    /// Does this system have required Vulkan validation layers?
    pub fn has_layers() -> Option<&'static [&'static str]> {
        for layer in alexandria_graphics::GraphicsInstance::enumerate_all_layers().unwrap() {
            if layer.name() == "VK_LAYER_KHRONOS_validation" {
                return Some(&["VK_LAYER_KHRONOS_validation"]);
            }
        }

        None
    }

    /// Does this system have required Vulkan validation extensions?
    pub fn has_extensions() -> Option<&'static [alexandria_graphics::GraphicsInstanceExtension]> {
        for extension in alexandria_graphics::GraphicsInstance::enumerate_extensions().unwrap() {
            if extension == alexandria_graphics::GraphicsInstanceExtension::DebugUtils {
                return Some(&[alexandria_graphics::GraphicsInstanceExtension::DebugUtils]);
            }
        }

        None
    }

    /// Create a new debug messenger
    pub fn create(
        instance: &alexandria_graphics::GraphicsInstance,
    ) -> alexandria_graphics::GraphicsDebugMessenger<DebugCallback> {
        instance
            .create_debug_messenger(
                alexandria_graphics::GraphicsDebugMessageSeverity::Verbose,
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
    pub fn has_layers() -> Option<&'static [&'static str]> {
        Some(&[])
    }

    /// Does this system have required Vulkan validation extensions?
    pub fn has_extensions() -> Option<&'static [alexandria_graphics::GraphicsInstanceExtension]> {
        Some(&[])
    }

    /// Create a new debug messenger
    pub fn create(_: &alexandria_graphics::GraphicsInstance) -> () {
        ()
    }
}
