#[test]
fn triangle() {
    let instance = alexandria::Instance::new("Triangle Test\0", 1).unwrap();

    let window = alexandria::Window::new(instance, "Triangle Test", 1920, 1080).unwrap();

    while window.poll_events().unwrap() {}
}
