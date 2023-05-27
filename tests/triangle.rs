#[test]
fn triangle() {
    let instance = alexandria::Instance::new(
        "Triangle Test".to_owned(),
        alexandria::Version::new(0, 0, 0, 0),
    )
    .unwrap();
    let mut window = instance.create_window("Triangle Test", 1920, 1080).unwrap();

    let device = window.enumerate_devices().unwrap().swap_remove(0);
    let graphics_context = window.create_graphics_context(device).unwrap();

    while window.poll_events().unwrap() {}
}
