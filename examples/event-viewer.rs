use render_context::RenderContext;

use crate::render_context::Swapchain;

mod render_context;

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

    let (window, mut render_context, surface) = if create_window {
        let window = context
            .window()
            .create_window("Event Viewer")
            .minimum_size(Some((800, 600).into()))
            .resizable()
            .create()
            .unwrap();

        let (render_context, surface) = RenderContext::new(&context, &window);

        (Some(window), Some(render_context), Some(surface))
    } else {
        (None, None, None)
    };

    let mut swapchain = if let Some(window) = &window {
        Some(Swapchain::new(
            render_context.as_ref().unwrap(),
            surface.as_ref().unwrap(),
            window,
        ))
    } else {
        None
    };

    let mut running = true;
    let mut should_recreate_swapchain = false;
    while running {
        let rendered = if let Some(swapchain) = &mut swapchain {
            if !window.as_ref().unwrap().is_minimized() {
                let rendered = swapchain.render_frame(render_context.as_mut().unwrap());
                if !rendered {
                    should_recreate_swapchain = true;
                }
                rendered
            } else {
                true
            }
        } else {
            true
        };

        if rendered {
            let event = pump.wait().unwrap();
            running &= handle_event(&event, &context, &mut should_recreate_swapchain);
        }

        while let Some(event) = pump.poll().unwrap() {
            running &= handle_event(&event, &context, &mut should_recreate_swapchain);
        }

        // Recreate the swapchain when appropriate
        if should_recreate_swapchain && !window.as_ref().unwrap().is_minimized() {
            should_recreate_swapchain = false;

            render_context.as_mut().unwrap().wait_idle();

            drop(swapchain);
            swapchain = Some(Swapchain::new(
                render_context.as_ref().unwrap(),
                surface.as_ref().unwrap(),
                window.as_ref().unwrap(),
            ));
        }
    }

    if let Some(window) = window {
        render_context.as_mut().unwrap().wait_idle();
        window.destroy().unwrap();
    }
}

fn handle_event(
    event: &alexandria::Event<()>,
    context: &alexandria::AlexandriaContext<()>,
    should_recreate_swapchain: &mut bool,
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
        alexandria::EventKind::DisplayContentScaleChanged {
            id,
            new_content_scale,
        } => {
            println!(
                "[DISPLAY][SCALE CHANGED] {} to ({}%)",
                id,
                (new_content_scale * 100.0).round(),
            );
        }

        alexandria::EventKind::WindowCloseRequest { id } => {
            println!("[WINDOW][CLOSE REQUEST] {}", id);
            context.window().destroy_window(id).unwrap();
        }
        alexandria::EventKind::WindowResized { id, new_size } => {
            println!("[WINDOW][RESIZED] {} to {}x{}", id, new_size.x, new_size.y);
            *should_recreate_swapchain = true;
        }
        alexandria::EventKind::WindowMoved { id, new_position } => {
            println!("[WINDOW][MOVED] {} to {}", id, new_position);
        }
        alexandria::EventKind::WindowMinimized { id } => {
            println!("[WINDOW][MINIMIZED] {}", id);
        }
        alexandria::EventKind::WindowMaximized { id } => {
            println!("[WINDOW][MAXIMIZED] {}", id);
        }
        alexandria::EventKind::WindowRestored { id } => {
            println!("[WINDOW][RESTORED] {}", id);
        }
        alexandria::EventKind::WindowGainedFocus { id } => {
            println!("[WINDOW][GAINED FOCUS] {}", id);
        }
        alexandria::EventKind::WindowLostFocus { id } => {
            println!("[WINDOW][LOST FOCUS] {}", id);
        }
        alexandria::EventKind::WindowShown { id } => {
            println!("[WINDOW][SHOWN] {}", id);
        }
        alexandria::EventKind::WindowHidden { id } => {
            println!("[WINDOW][HIDDEN] {}", id);
        }
        alexandria::EventKind::WindowContentScaleChanged {
            id,
            new_content_scale,
        } => {
            println!(
                "[WINDOW][SCALE CHANGED] {} to ({}%)",
                id,
                (new_content_scale * 100.0).round(),
            );
        }
        alexandria::EventKind::WindowEnteredFullscreen { id } => {
            println!("[WINDOW][ENTERED FULLSCREEN] {}", id);
        }
        alexandria::EventKind::WindowLeftFullscreen { id } => {
            println!("[WINDOW][LEFT FULLSCREEN] {}", id);
        }
        alexandria::EventKind::WindowDestroyed { id } => {
            println!("[WINDOW][DESTROYED] {}", id);
            context
                .event_queue()
                .push(alexandria::EventKind::Quit)
                .unwrap();
        }

        alexandria::EventKind::KeyDown {
            window_id,
            key_code,
            scan_code,
            is_repeat,
            ..
        } => {
            println!(
                "[KEY DOWN] {} (window: {}, scan code: {}, repeat: {})",
                key_code, window_id, scan_code, is_repeat
            );
        }
        alexandria::EventKind::KeyUp {
            window_id,
            key_code,
            scan_code,
            ..
        } => {
            println!(
                "[KEY UP] {} (window: {}, scan code: {})",
                key_code, window_id, scan_code
            );
        }

        alexandria::EventKind::User(_) => println!("[USER]"),
    }

    true
}
