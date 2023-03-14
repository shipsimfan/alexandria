use crate::Adapter;
use win32::{string_to_utf16, D3D12Device, D3D12Object, Interface};

type Result<T> = core::result::Result<T, win32::Win32Error>;

pub(super) struct Device {
    device: win32::ID3D12Device,
}

impl Device {
    pub(super) fn new(adapter: &mut Adapter) -> Result<Self> {
        let mut device: win32::ID3D12Device =
            win32::d3d12_create_device(adapter.inner(), win32::D3DFeatureLevel::_12_1)?;
        device.set_name(&string_to_utf16!("D3D12 Device"))?;

        Ok(Device { device })
    }

    pub(super) fn get_debug(&mut self) -> Result<win32::ID3D12InfoQueue> {
        self.device.query_interface()
    }

    pub(super) fn create_command_queue(
        &mut self,
        desc: &win32::D3D12CommandQueueDesc,
    ) -> Result<win32::ID3D12CommandQueue> {
        self.device.create_command_queue(desc)
    }
}
