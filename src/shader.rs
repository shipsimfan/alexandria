use crate::Window;
use std::ffi::CString;
use win32::{DirectXError, ID3DBlob};

pub use win32::DXGIFormat as Format;

pub struct Shader {
    vertex_shader: win32::ID3D11VertexShader,
    pixel_shader: win32::ID3D11PixelShader,
    input_layout: win32::ID3D11InputLayout,
}

pub struct ShaderCreationError {
    error: DirectXError,
    blob: Option<ID3DBlob>,
}

impl Shader {
    pub fn new<S: AsRef<str>>(
        code: S,
        vertex_layout: &[(&str, Format)],
        window: &mut Window,
    ) -> Result<Self, ShaderCreationError> {
        let device = window.device();

        let shader_code = CString::new(code.as_ref()).unwrap();
        let (vertex_shader_blob, errors) = win32::d3d_compile(
            &shader_code,
            None,
            &[],
            Some(&CString::new("vertex_main").unwrap()),
            &CString::new("vs_5_0").unwrap(),
            &[win32::D3DCompileFlag::EnableStrictness],
            &[],
        );

        let (vertex_shader, vertex_shader_blob) = match vertex_shader_blob {
            Ok(blob) => (device.create_vertex_shader(&blob)?, blob),
            Err(error) => {
                return Err(ShaderCreationError {
                    error,
                    blob: errors,
                })
            }
        };

        let (pixel_shader_blob, errors) = win32::d3d_compile(
            &shader_code,
            None,
            &[],
            Some(&CString::new("pixel_main").unwrap()),
            &CString::new("ps_5_0").unwrap(),
            &[win32::D3DCompileFlag::EnableStrictness],
            &[],
        );

        let pixel_shader = match pixel_shader_blob {
            Ok(blob) => device.create_pixel_shader(&blob)?,
            Err(error) => {
                return Err(ShaderCreationError {
                    error,
                    blob: errors,
                })
            }
        };

        let mut input_layout_desc = Vec::with_capacity(vertex_layout.len());
        let mut names = Vec::with_capacity(vertex_layout.len());
        for (name, format) in vertex_layout {
            let i = names.len();
            names.push(CString::new(*name).unwrap());

            input_layout_desc.push(win32::D3D11InputElementDesc::new(
                &names[i],
                0,
                *format,
                0,
                None,
                win32::D3D11InputClassification::PerVertexData,
                0,
            ))
        }

        let input_layout =
            device.create_input_layout(input_layout_desc.as_slice(), &vertex_shader_blob)?;

        Ok(Shader {
            vertex_shader,
            pixel_shader,
            input_layout,
        })
    }

    pub fn set_active_shader(&mut self, window: &mut Window) {
        let dc = window.device_context();
        dc.ia_set_input_layout(&mut self.input_layout);
        dc.vs_set_shader(&mut self.vertex_shader);
        dc.ps_set_shader(&mut self.pixel_shader);
    }
}

impl std::error::Error for ShaderCreationError {}

impl std::fmt::Display for ShaderCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.blob {
                Some(errors) => format!("{}", errors),
                None => format!("{}", self.error),
            }
        )
    }
}

impl std::fmt::Debug for ShaderCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<win32::DirectXError> for ShaderCreationError {
    fn from(error: win32::DirectXError) -> Self {
        ShaderCreationError { error, blob: None }
    }
}
