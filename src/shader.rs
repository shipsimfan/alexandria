use crate::{Input, Window};
use std::{ffi::CString, marker::PhantomData, mem::size_of};
use win32::{DirectXError, ID3DBlob};

pub use win32::DXGIFormat as Format;

pub struct Shader {
    vertex_shader: win32::ID3D11VertexShader,
    pixel_shader: win32::ID3D11PixelShader,
    input_layout: win32::ID3D11InputLayout,
}

pub struct ShaderCB<T: Sized> {
    shader: Shader,
    constant_buffer: win32::ID3D11Buffer,
    phantom: PhantomData<T>,
}

pub struct ShaderCB2<T1: Sized, T2: Sized> {
    shader: Shader,
    constant_buffer1: win32::ID3D11Buffer,
    constant_buffer2: win32::ID3D11Buffer,
    phantom1: PhantomData<T1>,
    phantom2: PhantomData<T2>,
}

pub struct ShaderCreationError {
    error: DirectXError,
    blob: Option<ID3DBlob>,
}

#[derive(Debug)]
pub struct SetConstantBufferError(DirectXError);

impl Shader {
    pub fn new<S: AsRef<str>, I: Input>(
        code: S,
        vertex_layout: &[(&str, Format)],
        window: &mut Window<I>,
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

    pub fn set_active_shader<I: Input>(&mut self, window: &mut Window<I>) {
        let dc = window.device_context();
        dc.ia_set_input_layout(&mut self.input_layout);
        dc.vs_set_shader(&mut self.vertex_shader);
        dc.ps_set_shader(&mut self.pixel_shader);
    }
}

impl<T: Sized> ShaderCB<T> {
    pub fn new<S: AsRef<str>, I: Input>(
        code: S,
        vertex_layout: &[(&str, Format)],
        initial_data: Option<T>,
        window: &mut Window<I>,
    ) -> Result<Self, ShaderCreationError> {
        let buffer_desc = win32::D3D11BufferDesc::new(
            size_of::<T>() as u32,
            win32::D3D11Usage::Dynamic,
            &[win32::D3D11BindFlag::ConstantBuffer],
            &[win32::D3D11CPUAccessFlag::Write],
            &[],
            0,
        );

        let initial_data = match initial_data {
            Some(initial_data) => {
                let arr = [initial_data];
                Some(win32::D3D11SubresourceData::new(&arr, 0, 0))
            }
            None => None,
        };

        let buffer = window
            .device()
            .create_buffer(&buffer_desc, initial_data.as_ref())?;

        Ok(ShaderCB {
            shader: Shader::new(code, vertex_layout, window)?,
            constant_buffer: buffer,
            phantom: PhantomData,
        })
    }

    pub fn set_constant_buffer<I: Input>(
        &mut self,
        new_data: T,
        window: &mut Window<I>,
    ) -> Result<(), SetConstantBufferError> {
        let mut mapped_resource = window.device_context().map(
            &mut self.constant_buffer,
            0,
            win32::D3D11Map::WriteDiscard,
            &[],
        )?;

        let data = mapped_resource.as_ref::<T>();
        *data = new_data;

        Ok(())
    }

    pub fn set_active_shader<I: Input>(&mut self, window: &mut Window<I>) {
        self.shader.set_active_shader(window);
        window
            .device_context()
            .vs_set_constant_buffers(0, &mut [&mut self.constant_buffer]);
        window
            .device_context()
            .ps_set_constant_buffers(0, &mut [&mut self.constant_buffer]);
    }
}

impl<T1: Sized, T2: Sized> ShaderCB2<T1, T2> {
    pub fn new<S: AsRef<str>, I: Input>(
        code: S,
        vertex_layout: &[(&str, Format)],
        initial_data1: Option<T1>,
        initial_data2: Option<T2>,
        window: &mut Window<I>,
    ) -> Result<Self, ShaderCreationError> {
        let buffer_desc1 = win32::D3D11BufferDesc::new(
            size_of::<T1>() as u32,
            win32::D3D11Usage::Dynamic,
            &[win32::D3D11BindFlag::ConstantBuffer],
            &[win32::D3D11CPUAccessFlag::Write],
            &[],
            0,
        );

        let buffer_desc2 = win32::D3D11BufferDesc::new(
            size_of::<T2>() as u32,
            win32::D3D11Usage::Dynamic,
            &[win32::D3D11BindFlag::ConstantBuffer],
            &[win32::D3D11CPUAccessFlag::Write],
            &[],
            0,
        );

        let initial_data1 = match initial_data1 {
            Some(initial_data) => {
                let arr = [initial_data];
                Some(win32::D3D11SubresourceData::new(&arr, 0, 0))
            }
            None => None,
        };

        let initial_data2 = match initial_data2 {
            Some(initial_data) => {
                let arr = [initial_data];
                Some(win32::D3D11SubresourceData::new(&arr, 0, 0))
            }
            None => None,
        };

        let buffer1 = window
            .device()
            .create_buffer(&buffer_desc1, initial_data1.as_ref())?;

        let buffer2 = window
            .device()
            .create_buffer(&buffer_desc2, initial_data2.as_ref())?;

        Ok(ShaderCB2 {
            shader: Shader::new(code, vertex_layout, window)?,
            constant_buffer1: buffer1,
            constant_buffer2: buffer2,
            phantom1: PhantomData,
            phantom2: PhantomData,
        })
    }

    pub fn set_constant_buffer1<I: Input>(
        &mut self,
        new_data: T1,
        window: &mut Window<I>,
    ) -> Result<(), SetConstantBufferError> {
        let mut mapped_resource = window.device_context().map(
            &mut self.constant_buffer1,
            0,
            win32::D3D11Map::WriteDiscard,
            &[],
        )?;

        let data = mapped_resource.as_ref::<T1>();
        *data = new_data;

        Ok(())
    }

    pub fn set_constant_buffer2<I: Input>(
        &mut self,
        new_data: T2,
        window: &mut Window<I>,
    ) -> Result<(), SetConstantBufferError> {
        let mut mapped_resource = window.device_context().map(
            &mut self.constant_buffer2,
            0,
            win32::D3D11Map::WriteDiscard,
            &[],
        )?;

        let data = mapped_resource.as_ref::<T2>();
        *data = new_data;

        Ok(())
    }

    pub fn set_active_shader<I: Input>(&mut self, window: &mut Window<I>) {
        self.shader.set_active_shader(window);
        window.device_context().vs_set_constant_buffers(
            0,
            &mut [&mut self.constant_buffer1, &mut self.constant_buffer2],
        );
        window.device_context().ps_set_constant_buffers(
            0,
            &mut [&mut self.constant_buffer1, &mut self.constant_buffer2],
        );
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

impl std::error::Error for SetConstantBufferError {}

impl std::fmt::Display for SetConstantBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<win32::DirectXError> for SetConstantBufferError {
    fn from(error: win32::DirectXError) -> Self {
        SetConstantBufferError(error)
    }
}
