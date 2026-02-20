fn main() {
    let (context, mut pump) = alexandria::AlexandriaContext::<()>::builder()
        .gpu()
        .window()
        .create()
        .unwrap();

    let window = context
        .window()
        .create_window("Blank Window")
        .create()
        .unwrap();

    println!(
        "Window created at {} with size {}x{}",
        window.position(),
        window.width(),
        window.height()
    );

    let mut running = true;
    while running {
        let event = pump.wait().unwrap();
        running &= handle_event(&event, &context);

        while let Some(event) = pump.poll().unwrap() {
            running &= handle_event(&event, &context);
        }
    }

    window.destroy();
}

fn handle_event(event: &alexandria::Event<()>, _: &alexandria::AlexandriaContext<()>) -> bool {
    match event.kind {
        alexandria::EventKind::Quit | alexandria::EventKind::WindowCloseRequest { .. } => false,
        _ => true,
    }
}
