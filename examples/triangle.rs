use std::time::{Duration, Instant};

const SECOND: Duration = Duration::from_secs(1);

fn main() {
    // Create the window
    let mut window = alexandria::WindowBuilder::new("Triangle Example")
        .create()
        .unwrap();

    // Setup fps counter
    let mut frames = 0;
    let mut second_counter = Duration::from_secs(0);
    let mut last_frame = Instant::now();

    // Main loop
    while window.is_running() {
        window.process_inputs().unwrap();

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
        window.end_render().unwrap();
    }

    drop(window);
}
