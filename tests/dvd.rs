#[repr(C)]
struct Vertex {
    position: alexandria::Vector2,
    uv: alexandria::Vector2,
}

#[repr(C)]
#[derive(Clone)]
struct MatrixBuffer {
    world: alexandria::Matrix,
    projection: alexandria::Matrix,
    color: alexandria::Vector4,
}

const VERTICES: [Vertex; 4] = [
    Vertex {
        position: alexandria::Vector2::new(-0.55, -0.325),
        uv: alexandria::Vector2::new(0.0, 0.0),
    },
    Vertex {
        position: alexandria::Vector2::new(-0.55, 0.325),
        uv: alexandria::Vector2::new(0.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector2::new(0.55, 0.325),
        uv: alexandria::Vector2::new(1.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector2::new(0.55, -0.325),
        uv: alexandria::Vector2::new(1.0, 0.0),
    },
];

const INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];
const SPEED: f32 = 2.0;

const COLORS: [alexandria::Vector4; 6] = [
    alexandria::Vector4::new(0.0, 1.0, 1.0, 1.0),
    alexandria::Vector4::new(0.0, 0.0, 1.0, 1.0),
    alexandria::Vector4::new(1.0, 0.0, 0.0, 1.0),
    alexandria::Vector4::new(1.0, 1.0, 0.0, 1.0),
    alexandria::Vector4::new(1.0, 0.0, 1.0, 1.0),
    alexandria::Vector4::new(0.0, 1.0, 0.0, 1.0),
];

#[test]
fn dvd() {
    let image: ginger::Image<f32> = ginger::open_image("./tests/dvd_logo.qoi").unwrap();

    let mut window: alexandria::Window =
        alexandria::Window::new("DVD Logo", 1920, 1080, true).unwrap();

    let shader_code = std::fs::read_to_string("./tests/dvd.acsl").unwrap();
    let projection = alexandria::Matrix::translation(0.0, 0.0, 1.0)
        * alexandria::Matrix::orthographic(10.0, 10.0 * (9.0 / 16.0), 0.01, 10.0);

    let mut matrix_buffer = MatrixBuffer {
        world: alexandria::Matrix::identity(),
        projection,
        color: COLORS[0],
    };
    let mut shader = alexandria::Shader::new(
        shader_code,
        &[
            ("POSITION", alexandria::Format::R32A32Float),
            ("TEXCOORD", alexandria::Format::R32A32Float),
        ],
        &mut window,
    )
    .unwrap();
    let mut constant_buffer =
        alexandria::ConstantBuffer::new(matrix_buffer.clone(), 0, &mut window).unwrap();

    let mut mesh = alexandria::Mesh::new(&VERTICES, &INDICES, &mut window).unwrap();

    let mut texture =
        alexandria::Texture2D::new(&image, 0, alexandria::SampleType::Point, &mut window).unwrap();

    drop(image);

    let mut position = alexandria::Vector2::new(0.0, 0.0);
    let mut velocity = alexandria::Vector2::new(SPEED, SPEED);

    let mut last_tick = std::time::Instant::now();
    let mut current_color = 0;

    while window.poll_events() {
        window.begin_render([0.0, 0.0, 0.0, 1.0]);

        shader.set_active();
        constant_buffer.set_active();
        constant_buffer.set_data(matrix_buffer.clone()).unwrap();
        texture.set_active();
        mesh.render();

        window.end_render().unwrap();

        let current_tick = std::time::Instant::now();
        let dt = (current_tick - last_tick).as_secs_f32();
        last_tick = current_tick;

        position += velocity * dt;
        if position.x() <= -4.45 {
            position.set_x(-4.45);
            velocity.set_x(-velocity.x());

            current_color = (current_color + 1) % 6;
        } else if position.x() >= 4.45 {
            position.set_x(4.45);
            velocity.set_x(-velocity.x());

            current_color = (current_color + 1) % 6;
        }

        if position.y() <= -5.0 * (9.0 / 16.0) + 0.325 {
            position.set_y(-5.0 * (9.0 / 16.0) + 0.325);
            velocity.set_y(-velocity.y());

            current_color = (current_color + 1) % 6;
        } else if position.y() >= 5.0 * (9.0 / 16.0) - 0.325 {
            position.set_y(5.0 * (9.0 / 16.0) - 0.325);
            velocity.set_y(-velocity.y());

            current_color = (current_color + 1) % 6;
        }

        matrix_buffer.world = alexandria::Matrix::translation(position.x(), position.y(), 0.0);
        matrix_buffer.color = COLORS[current_color];
    }
}
