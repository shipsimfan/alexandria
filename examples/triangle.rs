use std::time::{Duration, Instant};

#[repr(C)]
struct Vertex {
    position: alexandria::math::Vector2f,
    color: alexandria::math::Color3f,
}

impl alexandria::graphics::Vertex for Vertex {
    const INPUT_LAYOUT: &[alexandria::graphics::InputElement] = &[
        alexandria::graphics::InputElement::new(
            "POSITION",
            0,
            alexandria::graphics::InputElementType::Vector2F32,
        ),
        alexandria::graphics::InputElement::new(
            "COLOR",
            0,
            alexandria::graphics::InputElementType::Vector3F32,
        ),
    ];
}

const SECOND: Duration = Duration::from_secs(1);

const SHADER: alexandria::acsl::D3DProgram<Vertex, ()> =
    alexandria::compile_hlsl_file!("triangle.hlsl", "vertex_main", "pixel_main");

const VERTICES: &[Vertex] = &[
    Vertex {
        position: alexandria::math::Vector2f::new(-0.5, -0.5),
        color: alexandria::math::Color3f::RED,
    },
    Vertex {
        position: alexandria::math::Vector2f::new(0.0, 0.5),
        color: alexandria::math::Color3f::GREEN,
    },
    Vertex {
        position: alexandria::math::Vector2f::new(0.5, -0.5),
        color: alexandria::math::Color3f::BLUE,
    },
];

const INDICES: &[u32] = &[1, 0, 2];

fn main() {
    // Create the window
    let mut window = alexandria::WindowBuilder::new("Triangle Example")
        .log_callbacks(alexandria::StdoutLogger)
        .create()
        .unwrap();

    // Run the window
    if let Err(error) = run(&mut window) {
        // Handle error
        window.get_debug_messages().ok();
        alexandria::message_box::message_box_ok(
            "Runtime Error",
            &error.to_string(),
            alexandria::message_box::MessageBoxStyle::Error,
            Some(&mut window),
        )
        .unwrap();
        panic!("{}", error);
    }

    // Display shutdown debug messages
    window.get_debug_messages().unwrap();
}

fn run(window: &mut Box<alexandria::Window<alexandria::StdoutLogger>>) -> alexandria::Result<()> {
    // Create render resources
    let mut shader = window.graphics_context().create_shader(&SHADER, &())?;
    let mut mesh = window.graphics_context().create_mesh(VERTICES, INDICES)?;

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
        let mut render_context = window.begin_render([0.0, 0.0, 0.0, 0.0])?;

        render_context.render(&mut mesh, &mut shader);

        render_context.end()?;
    }

    Ok(())
}
