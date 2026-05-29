use alexandria::gpu::{
    VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceFeatures, VulkanDeviceVulkan13Features,
};

/// Prints out information about the graphics hardware on the system
fn main() {
    let (context, _) = alexandria::AlexandriaContext::<()>::builder()
        .gpu()
        .create()
        .unwrap();

    println!("     --- Graphics Information ---");

    // Get Vulkan version
    let version = context.gpu().version().unwrap();
    println!("Vulkan Version: {}", version);

    // List graphics instance extensions
    let extensions = context.gpu().all_extensions(None).unwrap();
    print!("Vulkan Instance Extensions:");
    if extensions.len() == 0 {
        println!(" No Vulkan instance extensions supported");
    } else {
        println!();
        for extension in extensions {
            println!(" - {}", extension);
        }
    }

    // List graphics instance layers
    let layers = context.gpu().layers().unwrap();
    print!("Vulkan Instance Layers:");
    if layers.len() == 0 {
        println!(" No Vulkan instance layers supported");
    } else {
        println!();
        for layer in layers {
            println!(" - {} (Driver: v{})", layer, layer.version());
            if layer.description().len() > 0 {
                println!("     {}", layer.description());
            }

            let extensions = context.gpu().all_extensions(Some(&layer)).unwrap();

            if extensions.len() == 0 {
                continue;
            }

            println!("     Extensions:");
            for extension in extensions {
                println!("      - {}", extension);
            }
        }
    }

    // Create an instance for enumerating adapters
    let instance = context
        .gpu()
        .instance_builder(version)
        .application(
            "Enumerate Adapters Example",
            alexandria::gpu::VulkanVersion::new(0, 1, 0, 0),
        )
        .create()
        .unwrap();

    // List graphics adapters
    let adapters = instance.enumerate_adapters().unwrap();
    print!("Adapters:");
    if adapters.len() == 0 {
        println!(" No Vulkan adapters available");
    } else {
        println!();
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

            let extensions = adapter.enumerate_all_extensions().unwrap();
            if extensions.len() == 0 {
                println!("   - No extensions");
            } else {
                println!("   - Extensions:");
                for extension in extensions {
                    println!("     - {}", extension);
                }
            }

            println!("   - Extended Info:");
            let mut extended_info = [
                VulkanDeviceFeatures::default().into(),
                VulkanDeviceVulkan13Features::default().into(),
                VulkanDeviceExtendedDynamicStateFeatures::default().into(),
            ];
            adapter.get_extended_info(&mut extended_info);

            print!("     - Vulkan 1.3 Features: ");
            let vulkan_13_features = extended_info[1].as_vulkan_13_features().unwrap();
            if vulkan_13_features.synchronization2() {
                print!("synchronization2 ");
            }
            if vulkan_13_features.dynamic_rendering() {
                print!("dynamic_rendering");
            }
            println!();

            print!("     - Extended Dynamic State Features: ");
            let extended_dynamic_state_features = extended_info[2]
                .as_extended_dynamic_state_features()
                .unwrap();
            if extended_dynamic_state_features.extended_dynamic_state() {
                print!("extended_dynamic_state");
            }
            println!();
        }
    }
}
