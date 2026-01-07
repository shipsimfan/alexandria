const TITLE: &str = "Blank Window Example";

#[cfg(debug_assertions)]
const VALIDATION_LAYERS: &[alexandria_graphics::GraphicsInstanceLayer] =
    &[alexandria_graphics::GraphicsInstanceLayer::KhronosValidation];
#[cfg(not(debug_assertions))]
const VALIDATION_LAYERS: &[&str] = &[];

fn main() {
    // Create Vulkan instance
    let layers = if has_validation_layers() {
        VALIDATION_LAYERS
    } else {
        println!("Warning: missing validation layers");
        &[]
    };

    let instance = alexandria_graphics::GraphicsInstance::builder(
        alexandria_graphics::GraphicsVersion::VERSION_1_4,
    )
    .application(TITLE, alexandria_graphics::GraphicsVersion::VERSION_1_0)
    .layers(layers.into_iter().map(|l| *l))
    .create()
    .unwrap();

    // Create window
    let mut window = alexandria_graphics::window::Window::builder(TITLE)
        .build()
        .unwrap();

    // Main loop
    while !window.is_close_requested() {
        window.process_messages().unwrap();
    }
}

/// Does this system have Vulkan validation layers?
fn has_validation_layers() -> bool {
    if VALIDATION_LAYERS.len() == 0 {
        return true;
    }

    let layers = alexandria_graphics::GraphicsInstance::enumerate_layers().unwrap();

    let mut has_validation_layers = true;
    for validation_layer in VALIDATION_LAYERS {
        let mut found = false;
        for layer in &layers {
            if layer == validation_layer {
                found = true;
                break;
            }
        }

        if !found {
            has_validation_layers = false;
            break;
        }
    }

    has_validation_layers
}
