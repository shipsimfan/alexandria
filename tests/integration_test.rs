use std::f32::consts::PI;

#[repr(C)]
struct Vertex {
    position: alexandria::Vector3,
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
        position: alexandria::Vector3::new(1.0, -1.0, 1.0),
        color: alexandria::Vector4::new(1.0, 0.0, 0.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector3::new(1.0, -1.0, -1.0),
        color: alexandria::Vector4::new(0.0, 1.0, 0.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector3::new(1.0, 1.0, -1.0),
        color: alexandria::Vector4::new(0.0, 0.0, 1.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector3::new(1.0, 1.0, 1.0),
        color: alexandria::Vector4::new(1.0, 1.0, 0.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector3::new(-1.0, -1.0, 1.0),
        color: alexandria::Vector4::new(1.0, 0.0, 1.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector3::new(-1.0, -1.0, -1.0),
        color: alexandria::Vector4::new(0.0, 1.0, 1.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector3::new(-1.0, 1.0, -1.0),
        color: alexandria::Vector4::new(1.0, 1.0, 1.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector3::new(-1.0, 1.0, 1.0),
        color: alexandria::Vector4::new(0.0, 0.0, 0.0, 1.0),
    },
];
const INDICES: [u32; 36] = [
    4, 0, 3, 4, 3, 7, 0, 1, 2, 0, 2, 3, 1, 5, 6, 1, 6, 2, 5, 4, 7, 5, 7, 6, 7, 3, 2, 7, 2, 6, 0, 5,
    1, 0, 4, 5,
];

#[test]
fn test() {
    let mut window: Box<alexandria::Window> =
        alexandria::Window::new("Testing", 1920, 1080).unwrap();

    let shader_code = std::fs::read_to_string("./default.hlsl").unwrap();
    let translation = alexandria::Matrix::translation(0.0, 0.0, 3.0);
    let mut matrix_buffer = MatrixBuffer {
        world: translation.into(),
        view: alexandria::Matrix::identity().into(),
        projection: alexandria::Matrix::perspective(PI / 2.0, 16.0 / 9.0, 0.01, 100.0).into(),
    };
    let mut shader = alexandria::Shader::new(
        shader_code,
        &[
            ("POSITION", alexandria::Format::R32G32A32Float),
            ("COLOR", alexandria::Format::R32G32B32A32Float),
        ],
        &mut window,
    )
    .unwrap();
    let mut constant_buffer =
        alexandria::ConstantBuffer::<_, 0>::new(Some(matrix_buffer), &mut window).unwrap();

    let mut mesh = alexandria::Mesh::new(&VERTICES, &INDICES, &mut window).unwrap();

    let mut y_rotation = 0.0;
    let mut x_rotation = 0.0;
    while window.poll_message() {
        window.begin_render([0.0, 0.0, 0.0, 1.0]);

        shader.set_active_shader(&mut window);
        constant_buffer.set_active(&mut window);
        constant_buffer.set(matrix_buffer, &mut window).unwrap();
        mesh.render(&mut window);

        window.end_render().unwrap();

        if window.input().get_key(b'D') || window.input().get_key(win32::VK_RIGHT) {
            y_rotation += 0.01;
        }

        if window.input().get_key(b'A') || window.input().get_key(win32::VK_LEFT) {
            y_rotation -= 0.01;
        }

        if window.input().get_key(b'W') || window.input().get_key(win32::VK_UP) {
            x_rotation -= 0.01;
        }

        if window.input().get_key(b'S') || window.input().get_key(win32::VK_DOWN) {
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
