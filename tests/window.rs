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
        while let Some(message) = instance.pop_debug_message()? {
            println!("[{}] {}", message.level(), message.message());
        }
    }

    Ok(())
}
