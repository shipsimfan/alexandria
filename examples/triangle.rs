use render_context::{RenderContext, Swapchain};

mod render_context;

alexandria::gpu::compile_shader!(const SHADER = "triangle.slang", vert_main, frag_main);

fn main() {
    // Create the Alexandria context with GPU and window support
    let (context, mut pump) = alexandria::AlexandriaContext::<()>::builder()
        .gpu()
        .window()
        .create()
        .expect("unable to create Alexandria context");

    // Create a window
    let window = context
        .window()
        .create_window("Blank Window")
        .resizable()
        .create()
        .expect("unable to create window");
    println!(
        "Window created at {} with size {}x{}",
        window.position(),
        window.width(),
        window.height()
    );

    // Create the render context
    let (mut render_context, mut surface) = RenderContext::new(&context, &window);

    // Create the shader
    let shader = render_context.create_shader_module(&SHADER);

    // Create the swapchain and image views
    let mut swapchain = Swapchain::new(&mut render_context, &mut surface, &window);

    // Run the main event loop
    let mut running = true;
    let mut should_recreate_swapchain = false;
    while running {
        // Render a frame if possible
        let rendered = if !window.is_minimized() {
            let rendered = swapchain.render_frame(
                &mut render_context,
                alexandria::math::Color3f::<alexandria::math::Linear>::new(0.0, 0.0, 0.0),
                || {},
            );
            if !rendered {
                should_recreate_swapchain = true;
            }
            rendered
        } else {
            false
        };

        if rendered || window.is_minimized() {
            // Wait for the next event and handle it
            let event = pump.wait().expect("unable to wait for event");
            running &= handle_event(&event, &mut should_recreate_swapchain);
        }

        // Poll for any additional events that may have occurred while handling the previous event
        while let Some(event) = pump.poll().expect("unable to poll for event") {
            running &= handle_event(&event, &mut should_recreate_swapchain);
        }

        // Recreate the swapchain when appropriate
        if should_recreate_swapchain && !window.is_minimized() {
            should_recreate_swapchain = false;

            render_context.wait_idle();

            drop(swapchain);
            swapchain = Swapchain::new(&mut render_context, &mut surface, &window);
        }
    }

    render_context.wait_idle();
    drop(shader);

    window.destroy().expect("unable to destroy window");
}

/// Handles an event and returns whether the application should continue running
fn handle_event(event: &alexandria::Event<()>, should_recreate_swapchain: &mut bool) -> bool {
    match event.kind {
        alexandria::EventKind::Quit | alexandria::EventKind::WindowCloseRequest { .. } => false,
        alexandria::EventKind::WindowResized { .. } => {
            *should_recreate_swapchain = true;
            true
        }
        _ => true,
    }
}
