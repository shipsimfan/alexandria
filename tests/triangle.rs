/*#[repr(C)]
struct Vertex {
    position: alexandria::Vector4,
    color: alexandria::Vector4,
}

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: alexandria::Vector4::new(-0.5, -0.5, 0.0, 1.0),
        color: alexandria::Vector4::new(1.0, 0.0, 0.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector4::new(0.0, 0.5, 0.0, 1.0),
        color: alexandria::Vector4::new(0.0, 1.0, 0.0, 1.0),
    },
    Vertex {
        position: alexandria::Vector4::new(0.5, -0.5, 0.0, 1.0),
        color: alexandria::Vector4::new(0.0, 0.0, 1.0, 1.0),
    },
];
const INDICES: [u32; 3] = [0, 1, 2];

#[test]
fn triangle() {
    let mut window: alexandria::Window =
        alexandria::Window::new("Triangle Test", 1920, 1080, true).unwrap();

    let shader_code = std::fs::read_to_string("./tests/triangle.acsl").unwrap();
    let mut shader = alexandria::Shader::new(
        shader_code,
        &[
            ("POSITION", alexandria::Format::R32G32B32A32Float),
            ("COLOR", alexandria::Format::R32G32B32A32Float),
        ],
        &mut window,
    )
    .unwrap();

    let mut mesh = alexandria::Mesh::new(&VERTICES, &INDICES, &mut window).unwrap();

    while window.poll_events() {
        window.begin_render([0.0, 0.0, 0.0, 1.0]);

        shader.set_active();
        mesh.render();

        window.end_render().unwrap();
    }
}
*/
