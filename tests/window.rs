#[test]
fn window() -> Result<(), alexandria::Error> {
    let mut instance = alexandria::Instance::new()?;

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
    }

    Ok(())
}
