fn main() {
    let mut window = alexandria::Window::new("Triangle Example", 1280, 720, None).unwrap();

    while window.is_running() {
        window.process_inputs();
    }

    drop(window);
}
