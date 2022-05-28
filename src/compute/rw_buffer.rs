use crate::{Input, Window};
use std::marker::PhantomData;

pub struct RWBuffer<T> {
    buffer: win32::ID3D11Buffer,
    view: win32::ID3D11UnorderedAccessView,
    slot: usize,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
pub struct RWBufferCreationError(win32::DirectXError);

impl<T> RWBuffer<T> {
    pub fn new<I: Input>(
        initial_data: &[T],
        slot: usize,
        window: &mut Window<I>,
    ) -> Result<Self, RWBufferCreationError> {
        let buffer_desc = win32::D3D11BufferDesc::new(
            (std::mem::size_of::<T>() * initial_data.len()) as u32,
            win32::D3D11Usage::Default,
            &[win32::D3D11BindFlag::UnorderedAccess],
            &[],
            &[win32::D3D11ResourceMiscFlag::BufferStructured],
            std::mem::size_of::<T>() as u32,
        );

        let initial_data = win32::D3D11SubresourceData::new(initial_data, 0, 0);
        let mut buffer = window
            .device()
            .create_buffer(&buffer_desc, Some(&initial_data))?;

        let uav_desc =
            win32::D3D11UnorderedAccessViewDesc::new(win32::DXGIFormat::Unknown, &mut buffer);
        let view = window
            .device()
            .create_unordered_access_view(&mut buffer, &uav_desc)?;

        Ok(RWBuffer {
            buffer,
            view,
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
            .cs_set_unordered_access_views(self.slot as u32, &mut [&mut self.view])
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
