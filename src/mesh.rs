use crate::Window;
use alexandria_common::Mesh as CommonMesh;

#[cfg(target_os = "windows")]
type MeshType<V> = alexandria_dx11::Mesh<V>;

#[cfg(target_os = "windows")]
type LineMeshType<V> = alexandria_dx11::LineMesh<V>;

pub struct Mesh<V>(MeshType<V>);

pub struct LineMesh<V>(LineMeshType<V>);

impl<V> Mesh<V> {
    #[inline(always)]
    pub fn new(
        vertices: &[V],
        indices: &[u32],
        window: &mut Window,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Mesh(<MeshType<V> as CommonMesh<V>>::new(
            vertices,
            indices,
            window.inner(),
        )?))
    }

    #[inline(always)]
    pub fn render(&mut self) {
        self.0.render()
    }
}
