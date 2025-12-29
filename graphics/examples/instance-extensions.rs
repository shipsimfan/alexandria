fn main() {
    let extensions = alexandria_graphics::GraphicsInstance::enumerate_all_extensions(None).unwrap();

    if extensions.len() == 0 {
        println!("No Vulkan instance extensions supported");
        return;
    }

    println!("Supported Vulkan instance extensions:");
    for extension in extensions {
        println!(" - {}", extension);
    }
}
