use crate::Adapter;
use win32::{
    dxgi::{IDXGIAdapter1, DXGI_ADAPTER_DESC1, DXGI_ADAPTER_FLAG},
    try_hresult, ComPtr,
};

impl Adapter {
    /// Create a new [`Adapter`]
    pub(super) fn new(mut adapter: ComPtr<IDXGIAdapter1>) -> Result<Self, win32::Error> {
        let mut desc = DXGI_ADAPTER_DESC1::default();
        try_hresult!(adapter.get_desc1(&mut desc))?;

        let mut name_len = desc.description.len();
        for i in 0..desc.description.len() {
            if desc.description[i] == 0 {
                name_len = i;
                break;
            }
        }
        let name = String::from_utf16_lossy(&desc.description[..name_len]);

        let is_software = desc.flags & DXGI_ADAPTER_FLAG::Software as u32 != 0;

        Ok(Adapter {
            adapter,
            name,
            video_memory: desc.dedicated_video_memory,
            is_software,
        })
    }
}
