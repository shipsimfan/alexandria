fn main() {
    let mut window = alexandria::Window::new("Window Example", 1280, 720).unwrap();

    while window.poll_events() {}
}
