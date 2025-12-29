fn main() {
    let version = alexandria_graphics::GraphicsVersion::enumerate().unwrap();

    println!("Vulkan version supported: {}", version);
}
