use std::f32::consts::PI;

#[repr(C)]
struct Vertex {
    position: graphics::Vector3,
    color: graphics::Vector4,
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
        position: graphics::Vector3::new(1.0, -1.0, 1.0),
        color: graphics::Vector4::new(1.0, 0.0, 0.0, 1.0),
    },
    Vertex {
        position: graphics::Vector3::new(1.0, -1.0, -1.0),
        color: graphics::Vector4::new(0.0, 1.0, 0.0, 1.0),
    },
    Vertex {
        position: graphics::Vector3::new(1.0, 1.0, -1.0),
        color: graphics::Vector4::new(0.0, 0.0, 1.0, 1.0),
    },
    Vertex {
        position: graphics::Vector3::new(1.0, 1.0, 1.0),
        color: graphics::Vector4::new(1.0, 1.0, 0.0, 1.0),
    },
    Vertex {
        position: graphics::Vector3::new(-1.0, -1.0, 1.0),
        color: graphics::Vector4::new(1.0, 0.0, 1.0, 1.0),
    },
    Vertex {
        position: graphics::Vector3::new(-1.0, -1.0, -1.0),
        color: graphics::Vector4::new(0.0, 1.0, 1.0, 1.0),
    },
    Vertex {
        position: graphics::Vector3::new(-1.0, 1.0, -1.0),
        color: graphics::Vector4::new(1.0, 1.0, 1.0, 1.0),
    },
    Vertex {
        position: graphics::Vector3::new(-1.0, 1.0, 1.0),
        color: graphics::Vector4::new(0.0, 0.0, 0.0, 1.0),
    },
];
const INDICES: [u32; 36] = [
    4, 0, 3, 4, 3, 7, 0, 1, 2, 0, 2, 3, 1, 5, 6, 1, 6, 2, 5, 4, 7, 5, 7, 6, 7, 3, 2, 7, 2, 6, 0, 5,
    1, 0, 4, 5,
];

#[test]
fn test() {
    let mut window = graphics::Window::new("Testing", 1920, 1080).unwrap();

    let shader_code = std::fs::read_to_string("./default.hlsl").unwrap();
    let translation = graphics::Matrix::translation(0.0, 0.0, 3.0);
    let mut matrix_buffer = MatrixBuffer {
        world: translation.into(),
        view: graphics::Matrix::identity().into(),
        projection: graphics::Matrix::perspective(PI / 2.0, 16.0 / 9.0, 0.01, 100.0).into(),
    };
    let mut shader = graphics::ShaderCB::new(
        shader_code,
        &[
            ("POSITION", graphics::Format::R32G32A32Float),
            ("COLOR", graphics::Format::R32G32B32A32Float),
        ],
        Some(matrix_buffer),
        &mut window,
    )
    .unwrap();

    let mut mesh = graphics::Mesh::new(&VERTICES, &INDICES, &mut window).unwrap();

    let mut rotation = 0.0;
    while window.poll_message() {
        window.begin_render([0.0, 0.0, 0.0, 1.0]);

        shader.set_active_shader(&mut window);
        shader
            .set_constant_buffer(matrix_buffer, &mut window)
            .unwrap();
        mesh.render(&mut window);

        window.end_render().unwrap();

        rotation += 0.01;
        if rotation >= 2.0 * PI {
            rotation -= 2.0 * PI;
        }

        matrix_buffer.world = (translation * graphics::Matrix::rotation_y(rotation)).into();
    }
}
