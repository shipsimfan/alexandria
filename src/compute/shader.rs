use crate::{Input, ShaderCreationError, Window};
use std::ffi::CString;

pub struct ComputeShader {
    compute_shader: win32::ID3D11ComputeShader,
}

impl ComputeShader {
    pub fn new<S: AsRef<str>, I: Input>(
        code: S,
        window: &mut Window<I>,
    ) -> Result<Self, ShaderCreationError> {
        let device = window.device();

        let shader_code = CString::new(code.as_ref()).unwrap();
        let (shader_blob, errors) = win32::d3d_compile(
            &shader_code,
            None,
            &[],
            Some(&CString::new("compute_main").unwrap()),
            &CString::new("cs_5_0").unwrap(),
            &[win32::D3DCompileFlag::EnableStrictness],
            &[],
        );

        let compute_shader = match shader_blob {
            Ok(blob) => device.create_compute_shader(&blob)?,
            Err(error) => return Err(ShaderCreationError::new(error, errors)),
        };

        Ok(ComputeShader { compute_shader })
    }

    pub fn set_active<I: Input>(&mut self, window: &mut Window<I>) {
        window
            .device_context()
            .cs_set_shader(&mut self.compute_shader);
    }

    pub fn dispatch<I: Input>(
        &mut self,
        threads_x: usize,
        threads_y: usize,
        threads_z: usize,
        window: &mut Window<I>,
    ) {
        window
            .device_context()
            .dispatch(threads_x as u32, threads_y as u32, threads_z as u32);
    }
}
