fn main() {
    if let Err(error) = run() {
        eprintln!("{}: {}", error.title(), error);
        alexandria::message_box(error.title(), &error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn alexandria::Error>> {
    let mut window = alexandria::Window::new("Window Example", 1280, 720)?;

    while window.poll_events() {}

    Ok(())
}
