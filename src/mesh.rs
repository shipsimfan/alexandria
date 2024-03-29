use crate::Window;
use alexandria_common::{Input, LineMesh as CommonLineMesh, Mesh as CommonMesh};

#[cfg(target_os = "windows")]
type MeshType<V> = alexandria_dx11::Mesh<V>;

#[cfg(target_os = "linux")]
type MeshType<V> = alexandria_opengl::Mesh<V>;

#[cfg(target_os = "windows")]
type LineMeshType<V> = alexandria_dx11::LineMesh<V>;

#[cfg(target_os = "linux")]
type LineMeshType<V> = alexandria_opengl::LineMesh<V>;

pub struct Mesh<V>(MeshType<V>);

pub struct LineMesh<V>(LineMeshType<V>);

impl<V> Mesh<V> {
    #[inline(always)]
    pub fn new<I: Input>(
        vertices: &[V],
        indices: &[u32],
        window: &mut Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Mesh(<MeshType<V> as CommonMesh<V>>::new(
            vertices,
            indices,
            window.inner(),
        )?))
    }

    #[inline(always)]
    pub fn update_vertices<I: Input>(
        &mut self,
        vertices: &[V],
        window: &mut Window<I>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.0.update_vertices(vertices, window.inner())
    }

    #[inline(always)]
    pub fn update_indices<I: Input>(
        &mut self,
        indices: &[u32],
        window: &mut Window<I>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.0.update_indices(indices, window.inner())
    }

    #[inline(always)]
    pub fn render(&mut self) {
        self.0.render()
    }
}

impl<V> LineMesh<V> {
    #[inline(always)]
    pub fn new<I: Input>(
        vertices: &[V],
        strip: bool,
        window: &mut Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(LineMesh(<LineMeshType<V> as CommonLineMesh<V>>::new(
            vertices,
            strip,
            window.inner(),
        )?))
    }

    #[inline(always)]
    pub fn render(&mut self) {
        self.0.render()
    }
}
