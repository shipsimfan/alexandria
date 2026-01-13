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
        println!("   - Kind: {}", adapter.kind());
        println!("   - Memory: {}", adapter.vram());
        println!("   - UUID: {}", adapter.uuid());
        println!("   - Vulkan Version: v{}", adapter.api_version());
        println!("   - Driver Version: v{}", adapter.driver_version());
        println!("   - Queues Families:");
        for queue in adapter.queue_families() {
            print!("     {}. {} queues (", queue.index(), queue.count());
            let mut first = true;
            if queue.graphics() {
                print!("graphics");
                first = false;
            }
            if queue.compute() {
                if first {
                    first = false;
                } else {
                    print!(", ");
                }
                print!("compute");
            }
            if queue.transfer() {
                if !first {
                    print!(", ");
                }
                print!("transfer");
            }
            println!(")");
        }
    }
}
