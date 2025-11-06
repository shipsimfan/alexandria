#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
        .input(alexandria::input::StateTrackingInput::new(
            alexandria::StdoutLogger,
        ))
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

fn run(
    window: &mut Box<
        alexandria::Window<
            alexandria::StdoutLogger,
            alexandria::input::StateTrackingInput<alexandria::StdoutLogger>,
        >,
    >,
) -> alexandria::Result<()> {
    // Prepare state
    let mut rotation = alexandria::math::Vector3f::ZERO;
    let mut translation = alexandria::math::Vector3f::new(0.0, 0.0, 10.0);

    // Create matrices
    let mut window_size = window.size();
    let mut projection_matrix = alexandria::math::Matrix4x4f::perspective(
        window_size.x as f32 / window_size.y as f32,
        3.14 / 4.0,
        0.01,
        1000.0,
    );

    let mut translation_matrix = alexandria::math::Matrix4x4f::translation(translation);
    let mut rotation_matrix = alexandria::math::Matrix4x4f::euler_rotation(rotation);

    let mut composition_matrix = rotation_matrix * translation_matrix * projection_matrix;

    let mut shader = window
        .graphics_context()
        .create_shader(&SHADER, &composition_matrix)?;
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

        let delta_t = delta_t.as_secs_f32();

        // Update state
        let mut update_rotation = false;
        let mut update_translation = false;

        if window.input().key(alexandria::input::KeyCode::LeftShift)
            || window.input().key(alexandria::input::KeyCode::RightShift)
        {
            if window.input().key(alexandria::input::KeyCode::A)
                || window.input().key(alexandria::input::KeyCode::LeftArrow)
            {
                update_translation = true;
                translation.x -= delta_t;
            }
            if window.input().key(alexandria::input::KeyCode::D)
                || window.input().key(alexandria::input::KeyCode::RightArrow)
            {
                update_translation = true;
                translation.x += delta_t;
            }
            if window.input().key(alexandria::input::KeyCode::W)
                || window.input().key(alexandria::input::KeyCode::UpArrow)
            {
                update_translation = true;
                translation.z += delta_t;
            }
            if window.input().key(alexandria::input::KeyCode::S)
                || window.input().key(alexandria::input::KeyCode::DownArrow)
            {
                update_translation = true;
                translation.z -= delta_t;
            }
            if window.input().key(alexandria::input::KeyCode::Q) {
                update_translation = true;
                translation.y += delta_t;
            }
            if window.input().key(alexandria::input::KeyCode::E) {
                update_translation = true;
                translation.y -= delta_t;
            }
        } else {
            if window.input().key(alexandria::input::KeyCode::A)
                || window.input().key(alexandria::input::KeyCode::LeftArrow)
            {
                update_rotation = true;
                rotation.y += delta_t;
            }
            if window.input().key(alexandria::input::KeyCode::D)
                || window.input().key(alexandria::input::KeyCode::RightArrow)
            {
                update_rotation = true;
                rotation.y -= delta_t;
            }
            if window.input().key(alexandria::input::KeyCode::W)
                || window.input().key(alexandria::input::KeyCode::UpArrow)
            {
                update_rotation = true;
                rotation.x += delta_t;
            }
            if window.input().key(alexandria::input::KeyCode::S)
                || window.input().key(alexandria::input::KeyCode::DownArrow)
            {
                update_rotation = true;
                rotation.x -= delta_t;
            }
            if window.input().key(alexandria::input::KeyCode::Q) {
                update_rotation = true;
                rotation.z += delta_t;
            }
            if window.input().key(alexandria::input::KeyCode::E) {
                update_rotation = true;
                rotation.z -= delta_t;
            }
        }

        // Update rotation and translation matrices
        let mut update_composition = false;
        if update_rotation {
            update_composition = true;
            rotation_matrix = alexandria::math::Matrix4x4f::euler_rotation(rotation);
        }

        if update_translation {
            update_composition = true;
            translation_matrix = alexandria::math::Matrix4x4f::translation(translation);
        }

        // Update projection
        if window.size() != window_size {
            window_size = window.size();
            update_composition = true;
            projection_matrix = alexandria::math::Matrix4x4f::perspective(
                window_size.x as f32 / window_size.y as f32,
                3.14 / 4.0,
                0.01,
                1000.0,
            );
        }

        // Render
        let mut render_context = window.begin_render([0.0, 0.0, 0.0, 0.0])?;

        // Update composition matrix
        if update_composition {
            composition_matrix = rotation_matrix * translation_matrix * projection_matrix;
            shader
                .update_constant_buffer(&composition_matrix, &mut render_context)
                .unwrap();
        }

        render_context.render(&mut mesh, &mut shader);

        render_context.end()?;
    }

    Ok(())
}
