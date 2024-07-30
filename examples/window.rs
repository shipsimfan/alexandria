use std::borrow::Cow;

fn main() {
    if let Err(error) = run() {
        eprintln!("{}: {}", error.title(), error);
        alexandria::message_box(error.title(), &error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn alexandria::Error>> {
    let instance = alexandria::Instance::new(Some(Box::new(log_callback)))?;

    let mut window = alexandria::Window::new("Window Example", 1280, 720)?;

    let physical_devices = instance.physical_devices(&window)?;
    println!("Available devices:");
    for physical_device in physical_devices {
        println!(" - {}", physical_device.name());
    }

    while window.poll_events() {}

    drop(instance);
    Ok(())
}

fn log_callback<'a, 'b>(severity: alexandria::Severity, message: &str, objects: Vec<Cow<str>>) {
    eprint!("[{}] {}", severity, message);
    if objects.len() > 0 {
        eprint!(" ({:?})", objects);
    }
    eprintln!();
}
