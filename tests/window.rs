#[test]
fn window() -> Result<(), alexandria::Error> {
    #[cfg(debug_assertions)]
    let enable_debugging = true;
    #[cfg(not(debug_assertions))]
    let enable_debugging = false;

    let mut instance = alexandria::Instance::new(enable_debugging)?;

    let mut window = alexandria::Window::new(
        "Testing",
        Some(alexandria::Resolution::new(1920, 1080)),
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
    while let Some(message) = instance.pop_debug_message()? {
        print_debug_message(message)
    }

    while let Some(message) = window.pop_debug_message()? {
        print_debug_message(message)
    }

    Ok(())
}

fn print_debug_message(message: alexandria::DebugMessage) {
    println!("[{}] {}", message.level(), message.message());
}
