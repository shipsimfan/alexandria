use crate::graphics::Vertex;
use std::{marker::PhantomData, num::NonZeroU32};
use win32::{
    d3d11::{ID3D11Buffer, ID3D11InputLayout, ID3D11PixelShader, ID3D11VertexShader},
    ComPtr,
};

mod bind;
mod get;
mod new;
mod update_constant_buffer;

/// A shader program which can be used to
pub struct Shader<V: Vertex, CB> {
    /// The ID assigned by the graphics context which uniquely identifies this shader
    id: NonZeroU32,

    /// The vertex shader
    vertex_shader: ComPtr<ID3D11VertexShader>,

    /// The pixel shader
    pixel_shader: ComPtr<ID3D11PixelShader>,

    /// The input layout describing `V`
    input_layout: ComPtr<ID3D11InputLayout>,

    /// The constant buffer, if there are bytes associated
    constant_buffer: Option<ComPtr<ID3D11Buffer>>,

    /// The vertex type this shader is made for
    _vertex: PhantomData<V>,

    /// The constant buffer type this shader is made for
    _constant_buffer_type: PhantomData<CB>,
}
