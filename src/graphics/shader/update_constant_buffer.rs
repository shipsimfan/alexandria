use crate::{
    graphics::{RenderFrame, Shader, Vertex},
    Error, Result,
};
use win32::{
    d3d11::{D3D11_MAP, D3D11_MAPPED_SUBRESOURCE},
    try_hresult,
};

impl<V: Vertex, CB: Clone> Shader<V, CB> {
    /// Update the value of the constant buffer for this shader
    pub fn update_constant_buffer(&mut self, value: &CB, frame: &mut RenderFrame) -> Result<()> {
        let constant_buffer = match &mut self.constant_buffer {
            Some(constant_buffer) => constant_buffer,
            None => return Ok(()),
        };

        let mut mapped_subresource = D3D11_MAPPED_SUBRESOURCE::default();
        try_hresult!(frame.device_context().map(
            constant_buffer.as_mut(),
            0,
            D3D11_MAP::WriteDiscard,
            0,
            &mut mapped_subresource,
        ))
        .map_err(|error| Error::new_os("unable to map constant buffer", error))?;

        *unsafe { &mut *(mapped_subresource.data as *mut CB) } = value.clone();

        frame.device_context().unmap(constant_buffer.as_mut(), 0);

        Ok(())
    }
}
