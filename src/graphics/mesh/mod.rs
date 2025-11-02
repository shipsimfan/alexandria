use crate::graphics::Vertex;
use std::marker::PhantomData;
use win32::{d3d11::ID3D11Buffer, ComPtr};

mod new;
mod render;

/// A mesh on the GPU
pub struct Mesh<V: Vertex> {
    /// The number of indices that make up this mesh
    index_count: usize,

    /// The buffer holding the vertex data
    vertex_buffer: ComPtr<ID3D11Buffer>,

    /// The buffer holding the index list
    index_buffer: ComPtr<ID3D11Buffer>,

    /// The type of vertices this mesh holds
    _vertex: PhantomData<V>,
}
