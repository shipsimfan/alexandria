use crate::Adapter;
use win32::{
    dxgi::{IDXGIAdapter, DXGI_ADAPTER_DESC},
    try_hresult, ComPtr,
};

impl Adapter {
    /// Create a new [`Adapter`]
    pub(super) fn new(mut adapter: ComPtr<IDXGIAdapter>) -> Result<Self, win32::Error> {
        let mut desc = DXGI_ADAPTER_DESC::default();
        try_hresult!(adapter.get_desc(&mut desc))?;

        let mut name_len = desc.description.len();
        for i in 0..desc.description.len() {
            if desc.description[i] == 0 {
                name_len = i;
                break;
            }
        }
        let name = String::from_utf16_lossy(&desc.description[..name_len]);

        Ok(Adapter { adapter, name })
    }
}
