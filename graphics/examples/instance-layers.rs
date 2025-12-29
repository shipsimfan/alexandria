fn main() {
    let layers = alexandria_graphics::GraphicsInstance::enumerate_all_layers().unwrap();

    if layers.len() == 0 {
        println!("No Vulkan instance layers supported");
        return;
    }

    println!("Supported Vulkan instance layers:");
    for layer in layers {
        println!(" - {} (Driver: v{})", layer, layer.version());
        if layer.description().len() > 0 {
            println!("     {}", layer.description());
        }

        let extensions =
            alexandria_graphics::GraphicsInstance::enumerate_all_extensions(Some(&layer)).unwrap();

        if extensions.len() == 0 {
            continue;
        }

        println!("     Extensions:");
        for extension in extensions {
            println!("      - {}", extension);
        }
    }
}
