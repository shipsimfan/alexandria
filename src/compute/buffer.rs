use crate::{Input, Window};
use std::marker::PhantomData;

pub struct Buffer<T> {
    buffer: win32::ID3D11Buffer,
    srv: win32::ID3D11ShaderResourceView,
    uav: win32::ID3D11UnorderedAccessView,
    slot: usize,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
pub struct RWBufferCreationError(win32::DirectXError);

impl<T> Buffer<T> {
    pub fn new<I: Input>(
        initial_data: &[T],
        slot: usize,
        window: &mut Window<I>,
    ) -> Result<Self, RWBufferCreationError> {
        let buffer_desc = win32::D3D11BufferDesc::new(
            (std::mem::size_of::<T>() * initial_data.len()) as u32,
            win32::D3D11Usage::Default,
            &[
                win32::D3D11BindFlag::UnorderedAccess,
                win32::D3D11BindFlag::ShaderResource,
            ],
            &[],
            &[win32::D3D11ResourceMiscFlag::BufferStructured],
            std::mem::size_of::<T>() as u32,
        );

        let initial_data = win32::D3D11SubresourceData::new(initial_data, 0, 0);
        let mut buffer = window
            .device()
            .create_buffer(&buffer_desc, Some(&initial_data))
            .unwrap();

        let srv_desc =
            win32::D3D11ShaderResourceViewDesc::new(win32::DXGIFormat::Unknown, &mut buffer);
        let srv = window
            .device()
            .create_shader_resource_view(&mut buffer, &srv_desc)
            .unwrap();

        let uav_desc =
            win32::D3D11UnorderedAccessViewDesc::new(win32::DXGIFormat::Unknown, &mut buffer);
        let uav = window
            .device()
            .create_unordered_access_view(&mut buffer, &uav_desc)
            .unwrap();

        Ok(Buffer {
            buffer,
            srv,
            uav,
            slot,
            phantom: PhantomData,
        })
    }

    pub fn set_slot(&mut self, slot: usize) {
        self.slot = slot;
    }

    pub fn set_active<I: Input>(&mut self, window: &mut Window<I>) {
        window
            .device_context()
            .cs_set_shader_resources(self.slot as u32, &mut [Some(&mut self.srv)])
    }

    pub fn set_active_rw<I: Input>(&mut self, window: &mut Window<I>) {
        window
            .device_context()
            .cs_set_unordered_access_views(self.slot as u32, &mut [Some(&mut self.uav)])
    }

    pub fn buffer(&mut self) -> &mut win32::ID3D11Buffer {
        &mut self.buffer
    }
}

impl std::error::Error for RWBufferCreationError {}

impl std::fmt::Display for RWBufferCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<win32::DirectXError> for RWBufferCreationError {
    fn from(error: win32::DirectXError) -> Self {
        RWBufferCreationError(error)
    }
}
