fn main() {
    let mut args = std::env::args();
    args.next(); // skip executable name
    let mut create_window = true;
    for arg in args {
        if arg == "--no-window" {
            create_window = false;
        } else {
            eprintln!("Unknown argument: {}", arg);
            std::process::exit(1);
        }
    }

    let (context, mut pump) = alexandria::AlexandriaContext::<()>::builder()
        .window()
        .create()
        .unwrap();

    let window = if create_window {
        Some(
            context
                .window()
                .create_window("Event Viewer")
                .fullscreen()
                .create()
                .unwrap(),
        )
    } else {
        None
    };

    let mut running = true;
    while running {
        let event = pump.wait().unwrap();
        running &= handle_event(&event, &context);

        while let Some(event) = pump.poll().unwrap() {
            running &= handle_event(&event, &context);
        }
    }

    if let Some(window) = window {
        window.destroy();
    }
}

fn handle_event(
    event: &alexandria::Event<()>,
    context: &alexandria::AlexandriaContext<()>,
) -> bool {
    let time = event.time - context.start_time();
    print!("{:10.03} ", time.as_secs_f64());

    match event.kind {
        alexandria::EventKind::Quit => {
            println!("[QUIT]");
            return false;
        }

        alexandria::EventKind::DisplayAdded { id } => {
            let display = context.window().display(id).unwrap();

            println!(
                "[DISPLAY][ADDED] {}: {} ({}x{} @ {})",
                id,
                display.name(),
                display.width(),
                display.height(),
                display.position()
            );
        }
        alexandria::EventKind::DisplayRemoved { id } => {
            println!("[DISPLAY][REMOVED] {}", id);
        }
        alexandria::EventKind::DisplayMoved { id, new_position } => {
            println!("[DISPLAY][MOVED] {} to {}", id, new_position);
        }
        alexandria::EventKind::DisplayResized { id, new_size } => {
            println!("[DISPLAY][RESIZED] {} to {}x{}", id, new_size.x, new_size.y);
        }
        alexandria::EventKind::DisplayWorkAreaChanged { id, new_work_area } => {
            println!(
                "[DISPLAY][WORK AREA CHANGED] {}: {}x{} @ {}",
                id, new_work_area.size.x, new_work_area.size.y, new_work_area.position
            );
        }
        alexandria::EventKind::DisplayRefreshRateChanged {
            id,
            new_refresh_rate,
        } => {
            println!(
                "[DISPLAY][REFRESH RATE CHANGED] {} to {:.02}Hz",
                id,
                new_refresh_rate.as_f32()
            );
        }
        alexandria::EventKind::DisplayRotated {
            id,
            new_orientation,
        } => {
            println!("[DISPLAY][ROTATED] {} to {:?}", id, new_orientation);
        }
        alexandria::EventKind::DisplayDpiChanged { id, new_dpi } => {
            println!(
                "[DISPLAY][DPI CHANGED] {} to {} ({}%)",
                id,
                new_dpi,
                (new_dpi as f32 / 0.96).trunc()
            );
        }

        alexandria::EventKind::WindowCloseRequest { id } => {
            println!("[WINDOW][CLOSE REQUEST] {}", id);
            context
                .event_queue()
                .push(alexandria::EventKind::Quit)
                .unwrap();
        }

        alexandria::EventKind::User(_) => println!("[USER]"),
    }

    true
}
