use std::f32::consts::PI;

#[repr(C)]
struct Vertex {
    position: alexandria::Vector4,
    color: alexandria::Vector4,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MatrixBuffer {
    world: [f32; 4 * 4],
    view: [f32; 4 * 4],
    projection: [f32; 4 * 4],
}

const VERTICES: [Vertex; 8] = [
    Vertex {
        position: alexandria::Vector4::new(1.0, -1.0, 1.0, 1.0),
        color: alexandria::Vector4::new(1.0, 0.0, 0.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector4::new(1.0, -1.0, -1.0, 1.0),
        color: alexandria::Vector4::new(0.0, 1.0, 0.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector4::new(1.0, 1.0, -1.0, 1.0),
        color: alexandria::Vector4::new(0.0, 0.0, 1.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector4::new(1.0, 1.0, 1.0, 1.0),
        color: alexandria::Vector4::new(1.0, 1.0, 0.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector4::new(-1.0, -1.0, 1.0, 1.0),
        color: alexandria::Vector4::new(1.0, 0.0, 1.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector4::new(-1.0, -1.0, -1.0, 1.0),
        color: alexandria::Vector4::new(0.0, 1.0, 1.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector4::new(-1.0, 1.0, -1.0, 1.0),
        color: alexandria::Vector4::new(1.0, 1.0, 1.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector4::new(-1.0, 1.0, 1.0, 1.0),
        color: alexandria::Vector4::new(0.0, 0.0, 0.0, 1.0),
    },
];
const INDICES: [u32; 36] = [
    4, 0, 3, 4, 3, 7, 0, 1, 2, 0, 2, 3, 1, 5, 6, 1, 6, 2, 5, 4, 7, 5, 7, 6, 7, 3, 2, 7, 2, 6, 0, 5,
    1, 0, 4, 5,
];

#[test]
fn cube() {
    let mut window: alexandria::Window =
        alexandria::Window::new("Cube Test", 1920, 1080, true).unwrap();

    let shader_code = std::fs::read_to_string("./tests/cube.acsl").unwrap();
    let translation = alexandria::Matrix::translation(0.0, 0.0, 3.0);
    let mut matrix_buffer = MatrixBuffer {
        world: translation.into(),
        view: alexandria::Matrix::identity().into(),
        projection: alexandria::Matrix::perspective(PI / 2.0, 16.0 / 9.0, 0.01, 100.0).into(),
    };
    let mut shader = alexandria::Shader::new(
        shader_code,
        &[
            ("POSITION", alexandria::Format::R32G32B32A32Float),
            ("COLOR", alexandria::Format::R32G32B32A32Float),
        ],
        &mut window,
    )
    .unwrap();
    let mut constant_buffer =
        alexandria::ConstantBuffer::new(matrix_buffer, 0, &mut window).unwrap();

    let mut mesh = alexandria::Mesh::new(&VERTICES, &INDICES, &mut window).unwrap();

    let mut y_rotation = 0.0;
    let mut x_rotation = 0.0;
    while window.poll_events() {
        window.begin_render([0.0, 0.0, 0.0, 1.0]);

        shader.set_active();
        constant_buffer.set_active();
        constant_buffer.set_data(matrix_buffer).unwrap();
        mesh.render();

        window.end_render().unwrap();

        if window.input().get_key(alexandria::Key::D)
            || window.input().get_key(alexandria::Key::RightArrow)
        {
            y_rotation += 0.01;
        }

        if window.input().get_key(alexandria::Key::A)
            || window.input().get_key(alexandria::Key::LeftArrow)
        {
            y_rotation -= 0.01;
        }

        if window.input().get_key(alexandria::Key::W)
            || window.input().get_key(alexandria::Key::UpArrow)
        {
            x_rotation -= 0.01;
        }

        if window.input().get_key(alexandria::Key::S)
            || window.input().get_key(alexandria::Key::DownArrow)
        {
            x_rotation += 0.01;
        }

        if y_rotation <= -2.0 * PI {
            y_rotation += 2.0 * PI;
        }

        if y_rotation >= 2.0 * PI {
            y_rotation -= 2.0 * PI;
        }

        if x_rotation <= -2.0 * PI {
            x_rotation += 2.0 * PI;
        }

        if x_rotation >= 2.0 * PI {
            x_rotation -= 2.0 * PI;
        }

        matrix_buffer.world = (translation
            * alexandria::Matrix::rotation_y(y_rotation)
            * alexandria::Matrix::rotation_x(x_rotation))
        .into();
    }
}
