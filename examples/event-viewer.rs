use std::time::Instant;

fn handle_event(
    event: &alexandria::Event<()>,
    start: Instant,
    context: &alexandria::AlexandriaContext<()>,
) -> bool {
    let time = event.time - start;
    print!("{:10.03} ", time.as_secs_f64());

    match event.kind {
        alexandria::EventKind::Quit => {
            println!("[QUIT]");
            return true;
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

        alexandria::EventKind::User(_) => println!("[USER]"),
    }

    false
}

fn main() {
    let start = Instant::now();
    let (context, mut pump) = alexandria::AlexandriaContext::<()>::builder()
        .window()
        .create()
        .unwrap();

    loop {
        let event = pump.wait().unwrap();
        if handle_event(&event, start, &context) {
            break;
        }

        while let Some(event) = pump.poll().unwrap() {
            if handle_event(&event, start, &context) {
                break;
            }
        }
    }

    drop(context);
}
