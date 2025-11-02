use crate::{
    graphics::{Mesh, Vertex},
    Error, Result,
};
use std::marker::PhantomData;
use win32::{
    d3d11::{
        ID3D11Device, D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_SUBRESOURCE_DATA, D3D11_USAGE,
    },
    try_hresult, ComPtr,
};

impl<V: Vertex> Mesh<V> {
    /// Create a new [`Mesh`]
    pub(in crate::graphics) fn new(
        vertices: &[V],
        indices: &[u32],
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Validate index buffer length
        if indices.len() % 3 != 0 {
            return Err(Error::new("indices length is not a multiple of 3"));
        }

        // Validate indices are in range
        for index in indices {
            if *index as usize >= vertices.len() {
                return Err(Error::new(format!(
                    "index {} is out of range for vertices of length {}",
                    *index,
                    vertices.len()
                )));
            }
        }

        // Create the mesh
        unsafe { Mesh::new_unchecked(vertices, indices, device) }
    }

    /// Create a new [`Mesh`] without validating the elements
    pub(in crate::graphics) unsafe fn new_unchecked(
        vertices: &[V],
        indices: &[u32],
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Create the vertex buffer
        let vertex_buffer_desc = D3D11_BUFFER_DESC {
            byte_width: (vertices.len() * std::mem::size_of::<V>()) as _,
            usage: D3D11_USAGE::Immutable,
            bind_flags: D3D11_BIND_FLAG::VertexBuffer as _,
            cpu_access_flags: 0,
            misc_flags: 0,
            structure_byte_stride: std::mem::size_of::<V>() as _,
        };
        let vertex_data = D3D11_SUBRESOURCE_DATA {
            sys_mem: vertices.as_ptr().cast(),
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        };
        let vertex_buffer = ComPtr::new_in(|vertex_buffer| {
            try_hresult!(device.create_buffer(&vertex_buffer_desc, &vertex_data, vertex_buffer))
        })
        .map_err(|error| Error::new_os("unable to create vertex buffer", error))?;

        // Create the index buffer
        let index_buffer_desc = D3D11_BUFFER_DESC {
            byte_width: (indices.len() * std::mem::size_of::<u32>()) as _,
            usage: D3D11_USAGE::Immutable,
            bind_flags: D3D11_BIND_FLAG::IndexBuffer as _,
            cpu_access_flags: 0,
            misc_flags: 0,
            structure_byte_stride: std::mem::size_of::<u32>() as _,
        };
        let index_data = D3D11_SUBRESOURCE_DATA {
            sys_mem: indices.as_ptr().cast(),
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        };
        let index_buffer = ComPtr::new_in(|index_buffer| {
            try_hresult!(device.create_buffer(&index_buffer_desc, &index_data, index_buffer))
        })
        .map_err(|error| Error::new_os("unable to create index buffer", error))?;

        Ok(Mesh {
            index_count: indices.len(),
            vertex_buffer,
            index_buffer,
            _vertex: PhantomData,
        })
    }
}
