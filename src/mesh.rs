use crate::{Device, Input, Window};
use std::{marker::PhantomData, sync::Arc};

pub enum Topology {
    Triangle,
    Line,
}

pub struct Mesh<V> {
    vertex_buffer: win32::ID3D11Buffer,
    index_buffer: win32::ID3D11Buffer,
    index_count: u32,
    topology: Topology,
    _phantom: PhantomData<V>,
}

#[derive(Debug)]
pub struct MeshCreationError(win32::DirectXError);

impl<V> Mesh<V> {
    pub fn new_with_device(
        vertices: &[V],
        indices: &[u32],
        device: &Arc<Device>,
    ) -> Result<Self, MeshCreationError> {
        let vertex_buffer_desc = win32::D3D11BufferDesc::new(
            (std::mem::size_of::<V>() * vertices.len()) as u32,
            win32::D3D11Usage::Default,
            &[win32::D3D11BindFlag::VertexBuffer],
            &[],
            &[],
            0,
        );

        let vertex_data = win32::D3D11SubresourceData::new(vertices, 0, 0);

        let vertex_buffer = device.create_buffer(&vertex_buffer_desc, Some(&vertex_data))?;

        let index_buffer_desc = win32::D3D11BufferDesc::new(
            (std::mem::size_of::<u32>() * indices.len()) as u32,
            win32::D3D11Usage::Default,
            &[win32::D3D11BindFlag::IndexBuffer],
            &[],
            &[],
            0,
        );

        let index_data = win32::D3D11SubresourceData::new(indices, 0, 0);

        let index_buffer = device.create_buffer(&index_buffer_desc, Some(&index_data))?;

        Ok(Mesh {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            topology: Topology::Line,
            _phantom: PhantomData,
        })
    }

    pub fn new<I: Input>(
        vertices: &[V],
        indices: &[u32],
        window: &mut Window<I>,
    ) -> Result<Self, MeshCreationError> {
        Mesh::new_with_device(vertices, indices, window.device())
    }

    pub fn set_topology(&mut self, topology: Topology) {
        self.topology = topology;
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        let dc = window.device_context();

        match self.topology {
            Topology::Triangle => {
                dc.ia_set_primitive_topology(win32::D3D11PrimitiveTopology::TriangleList)
            }
            Topology::Line => dc.ia_set_primitive_topology(win32::D3D11PrimitiveTopology::LineList),
        }

        dc.ia_set_vertex_buffers(
            0,
            &mut [&mut self.vertex_buffer],
            &[std::mem::size_of::<V>() as u32],
            &[0],
        );
        dc.ia_set_index_buffer(&mut self.index_buffer, win32::DXGIFormat::R32Uint, 0);

        dc.draw_indexed(self.index_count, 0, 0);
    }
}

impl std::error::Error for MeshCreationError {}

impl std::fmt::Display for MeshCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<win32::DirectXError> for MeshCreationError {
    fn from(error: win32::DirectXError) -> Self {
        MeshCreationError(error)
    }
}
