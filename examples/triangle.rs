fn main() {
    let mut window = alexandria::WindowBuilder::new("Triangle Example")
        .create()
        .unwrap();

    while window.is_running() {
        window.process_inputs().unwrap();

        window.end_render().unwrap();
    }

    drop(window);
}
