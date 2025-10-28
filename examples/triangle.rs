use std::time::{Duration, Instant};

const SECOND: Duration = Duration::from_secs(1);

fn main() {
    // Create the window
    let mut window = alexandria::WindowBuilder::new("Triangle Example")
        .log_callbacks(alexandria::StdoutLogger)
        .create()
        .unwrap();

    if let Err(error) = run(&mut window) {
        window.get_debug_messages().unwrap();
        panic!("{}", error);
    }
}

fn run(window: &mut Box<alexandria::Window<alexandria::StdoutLogger>>) -> alexandria::Result<()> {
    // Setup fps counter
    let mut frames = 0;
    let mut second_counter = Duration::from_secs(0);
    let mut last_frame = Instant::now();

    // Main loop
    while window.is_running() {
        window.process_inputs()?;

        // Calculate FPS
        let frame_time = Instant::now();
        let delta_t = frame_time - last_frame;

        frames += 1;
        second_counter += delta_t;
        last_frame = frame_time;

        if second_counter > SECOND {
            println!("FPS: {}", frames);
            frames = 0;
            second_counter -= SECOND;
        }

        // Render
        let render_context = window.begin_render([1.0, 0.0, 1.0, 0.0])?;

        render_context.end()?;
    }

    Ok(())
}
