#[repr(C)]
struct Vertex {
    position: graphics::Vector3,
    color: graphics::Vector4,
}

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: graphics::Vector3::new(-0.5, -0.5, 0.0),
        color: graphics::Vector4::new(1.0, 0.0, 0.0, 1.0),
    },
    Vertex {
        position: graphics::Vector3::new(0.0, 0.5, 0.0),
        color: graphics::Vector4::new(0.0, 1.0, 0.0, 1.0),
    },
    Vertex {
        position: graphics::Vector3::new(0.5, -0.5, 0.0),
        color: graphics::Vector4::new(0.0, 0.0, 1.0, 1.0),
    },
];
const INDICES: [u32; 3] = [0, 1, 2];

#[test]
fn test() {
    let mut window = graphics::Window::new("Testing", 1920, 1080).unwrap();

    let shader_code = std::fs::read_to_string("./default.hlsl").unwrap();
    let mut shader = graphics::Shader::new(
        shader_code,
        &[
            ("POSITION", graphics::Format::R32G32A32Float),
            ("COLOR", graphics::Format::R32G32B32A32Float),
        ],
        &mut window,
    )
    .unwrap();

    let mut mesh = graphics::Mesh::new(&VERTICES, &INDICES, &mut window).unwrap();

    while window.poll_message() {
        window.begin_render([0.0, 0.0, 0.0, 1.0]);

        shader.set_active_shader(&mut window);
        mesh.render(&mut window);

        window.end_render().unwrap();
    }
}
