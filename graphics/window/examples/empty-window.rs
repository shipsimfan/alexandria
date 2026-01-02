use alexandria_window::DisplayMode;

fn main() {
    let mut window = alexandria_window::Window::new(
        "Empty Window Example",
        None,
        None,
        None,
        None,
        DisplayMode::default(),
    )
    .unwrap();

    while window.is_running() {
        window.wait_for_message().unwrap();
        window.process_messages(None).unwrap();
    }
}
