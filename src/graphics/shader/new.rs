use crate::{
    graphics::{Shader, Vertex},
    Error, Result,
};
use acsl::D3DProgram;
use std::{ffi::CString, marker::PhantomData, num::NonZeroU32, ptr::null_mut};
use win32::{
    d3d11::{
        ID3D11Device, D3D11_APPEND_ALIGNED_ELEMENT, D3D11_INPUT_CLASSIFICATION,
        D3D11_INPUT_ELEMENT_DESC,
    },
    try_hresult, ComPtr,
};

impl<V: Vertex> Shader<V> {
    /// Create a new [`Shader`]
    pub(in crate::graphics) fn new(
        id: NonZeroU32,
        compiled_shader: &D3DProgram<V>,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Create vertex shader
        let vertex_shader = ComPtr::new_in(|vertex_shader| {
            try_hresult!(device.create_vertex_shader(
                compiled_shader.vertex_content().as_ptr().cast(),
                compiled_shader.vertex_content().len() as _,
                null_mut(),
                vertex_shader
            ))
        })
        .map_err(|error| Error::new_os("unable to create vertex shader", error))?;

        // Create pixel shader
        let pixel_shader = ComPtr::new_in(|pixel_shader| {
            try_hresult!(device.create_pixel_shader(
                compiled_shader.pixel_content().as_ptr().cast(),
                compiled_shader.pixel_content().len() as _,
                null_mut(),
                pixel_shader
            ))
        })
        .map_err(|error| Error::new_os("unable to create pixel shader", error))?;

        // Convert input layout
        let mut semantic_names = Vec::with_capacity(V::INPUT_LAYOUT.len());
        for input_element in V::INPUT_LAYOUT {
            semantic_names.push(CString::new(input_element.semantic_name()).unwrap());
        }

        let mut input_element_descs = Vec::with_capacity(V::INPUT_LAYOUT.len());
        for (i, input_element) in V::INPUT_LAYOUT.iter().enumerate() {
            input_element_descs.push(D3D11_INPUT_ELEMENT_DESC {
                semantic_name: semantic_names[i].as_ptr(),
                semantic_index: input_element.semantic_index() as _,
                format: input_element.r#type().to_d3d(),
                input_slot: 0,
                aligned_byte_offset: if i == 0 {
                    0
                } else {
                    D3D11_APPEND_ALIGNED_ELEMENT
                },
                input_slot_class: D3D11_INPUT_CLASSIFICATION::PerVertexData,
                instance_data_step_rate: 0,
            });
        }

        // Create input layout
        let input_layout = ComPtr::new_in(|input_layout| {
            try_hresult!(device.create_input_layout(
                input_element_descs.as_ptr(),
                input_element_descs.len() as _,
                compiled_shader.vertex_content().as_ptr().cast(),
                compiled_shader.vertex_content().len() as _,
                input_layout
            ))
        })
        .map_err(|error| Error::new_os("unable to create input layout", error))?;

        Ok(Shader {
            id,
            vertex_shader,
            pixel_shader,
            input_layout,
            _vertex: PhantomData,
        })
    }
}
