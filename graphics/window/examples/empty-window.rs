fn main() {
    let mut window = alexandria_window::Window::builder("Empty Window Example")
        .build()
        .unwrap();

    while !window.is_close_requested() {
        window.wait_for_message().unwrap();
        window.process_messages(None).unwrap();
    }
}
