fn main() {
    let instance = alexandria_graphics::GraphicsInstance::builder(
        alexandria_graphics::GraphicsVersion::VERSION_1_4,
    )
    .application(
        "Blank Window Example",
        alexandria_graphics::GraphicsVersion::new(0, 1, 0, 0),
    )
    .create()
    .unwrap();

    println!("Created instance!");
}
