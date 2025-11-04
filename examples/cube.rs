use std::time::{Duration, Instant};

#[repr(C)]
struct Vertex {
    position: alexandria::math::Vector3f,
    color: alexandria::math::Color3f,
}

impl alexandria::graphics::Vertex for Vertex {
    const INPUT_LAYOUT: &[alexandria::graphics::InputElement] = &[
        alexandria::graphics::InputElement::new(
            "POSITION",
            0,
            alexandria::graphics::InputElementType::Vector3F32,
        ),
        alexandria::graphics::InputElement::new(
            "COLOR",
            0,
            alexandria::graphics::InputElementType::Vector3F32,
        ),
    ];
}

const SECOND: Duration = Duration::from_secs(1);

const SHADER: alexandria::acsl::D3DProgram<Vertex, alexandria::math::Matrix4x4f> =
    alexandria::compile_hlsl_file!("cube.hlsl", "vertex_main", "pixel_main");

const VERTICES: &[Vertex] = &[
    Vertex {
        position: alexandria::math::Vector3f::new(-1.0, -1.0, -1.0),
        color: alexandria::math::Color3f::new(1.0, 0.0, 0.0),
    },
    Vertex {
        position: alexandria::math::Vector3f::new(1.0, -1.0, -1.0),
        color: alexandria::math::Color3f::new(0.0, 1.0, 0.0),
    },
    Vertex {
        position: alexandria::math::Vector3f::new(1.0, 1.0, -1.0),
        color: alexandria::math::Color3f::new(0.0, 0.0, 1.0),
    },
    Vertex {
        position: alexandria::math::Vector3f::new(-1.0, 1.0, -1.0),
        color: alexandria::math::Color3f::new(1.0, 1.0, 0.0),
    },
    Vertex {
        position: alexandria::math::Vector3f::new(-1.0, -1.0, 1.0),
        color: alexandria::math::Color3f::new(1.0, 0.0, 1.0),
    },
    Vertex {
        position: alexandria::math::Vector3f::new(1.0, -1.0, 1.0),
        color: alexandria::math::Color3f::new(0.0, 1.0, 1.0),
    },
    Vertex {
        position: alexandria::math::Vector3f::new(1.0, 1.0, 1.0),
        color: alexandria::math::Color3f::new(1.0, 1.0, 1.0),
    },
    Vertex {
        position: alexandria::math::Vector3f::new(-1.0, 1.0, 1.0),
        color: alexandria::math::Color3f::new(0.0, 0.0, 0.0),
    },
];

const INDICES: &[u32] = &[
    0, 1, 3, 3, 1, 2, 1, 5, 2, 2, 5, 6, 5, 4, 6, 6, 4, 7, 4, 0, 7, 7, 0, 3, 3, 2, 7, 7, 2, 6, 4, 5,
    0, 0, 5, 1,
];

fn main() {
    // Create the window
    let mut window = alexandria::WindowBuilder::new("Cube Example")
        .log_callbacks(alexandria::StdoutLogger)
        .vsync(false)
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
    let projection_matrix = alexandria::math::Matrix4x4f::perspective(
        window.width() as f32 / window.height() as f32,
        3.14 / 4.0,
        0.01,
        1000.0,
    );
    let translation_matrix =
        alexandria::math::Matrix4x4f::translation(alexandria::math::Vector3f::new(1.0, -1.0, 10.0));
    let x_rotation_matrix = alexandria::math::Matrix4x4f::x_rotation(3.14 / 4.0);

    let mut composition_matrix = x_rotation_matrix * translation_matrix * projection_matrix;

    let mut shader = window
        .graphics_context()
        .create_shader(&SHADER, &composition_matrix)?;
    let mut mesh = window.graphics_context().create_mesh(VERTICES, INDICES)?;

    // Setup fps counter
    let mut frames = 0;
    let mut second_counter = Duration::from_secs(0);
    let mut last_frame = Instant::now();

    // Setup animation
    let mut angle = 0.0;

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

        // Update matrix
        angle += delta_t.as_secs_f32();
        if angle > 2.0 * 3.14 {
            angle -= 2.0 * 3.14;
        }
        let y_rotation_matrix = alexandria::math::Matrix4x4f::y_rotation(angle);

        composition_matrix =
            x_rotation_matrix * y_rotation_matrix * translation_matrix * projection_matrix;

        // Render
        let mut render_context = window.begin_render([0.0, 0.0, 0.0, 0.0])?;

        shader
            .update_constant_buffer(&composition_matrix, &mut render_context)
            .unwrap();
        render_context.render(&mut mesh, &mut shader);

        render_context.end()?;
    }

    Ok(())
}
