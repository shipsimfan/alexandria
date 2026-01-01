const KILO: u64 = 1024;
const MEGA: u64 = KILO * 1024;
const GIGA: u64 = MEGA * 1024;

fn main() {
    let instance = alexandria_graphics::GraphicsInstance::builder(
        alexandria_graphics::GraphicsVersion::VERSION_1_4,
    )
    .application(
        "Enumerate Adapters Example",
        alexandria_graphics::GraphicsVersion::new(0, 1, 0, 0),
    )
    .create()
    .unwrap();

    let adapters = instance.enumerate_adapters().unwrap();
    if adapters.len() == 0 {
        println!("No adapters available!");
        return;
    }

    println!("Adapters:");
    for adapter in adapters {
        println!(" - {}", adapter.name());
        println!("    - Kind: {}", adapter.kind());
        print!("    - Memory: ");
        if adapter.vram() < KILO {
            println!("{}B", adapter.vram())
        } else if adapter.vram() < MEGA {
            println!("{}KB", adapter.vram() / KILO);
        } else if adapter.vram() < GIGA {
            println!("{}MB", adapter.vram() / MEGA);
        } else {
            println!("{}GB", adapter.vram() / GIGA);
        }
        println!("    - UUID: {}", adapter.uuid());
        println!("    - Vulkan Version: v{}", adapter.api_version());
        println!("    - Driver Version: v{}", adapter.driver_version());
    }
}
