#[test]
fn window() -> Result<(), alexandria::Error> {
    #[cfg(debug_assertions)]
    let enable_debugging = true;
    #[cfg(not(debug_assertions))]
    let enable_debugging = false;

    let mut instance = alexandria::Instance::new(enable_debugging)?;

    let mut window = alexandria::Window::new(
        "Testing",
        Some(alexandria::Resolution::new(1280, 720)),
        None,
        &mut instance,
        None,
        None,
    )?;

    while window.is_alive() {
        window.poll_events();

        #[cfg(debug_assertions)]
        print_debug_messages(&mut instance, &mut window)?;
    }

    Ok(())
}

fn print_debug_messages(
    instance: &mut alexandria::Instance,
    window: &mut alexandria::Window,
) -> alexandria::Result<()> {
    for message in instance.get_debug_messages()? {
        print_debug_message(message)
    }

    for message in window.graphics_3d().get_debug_messages()? {
        print_debug_message(message)
    }

    Ok(())
}

fn print_debug_message(message: alexandria::DebugMessage) {
    println!("[{}] {}", message.level(), message.message());
}
