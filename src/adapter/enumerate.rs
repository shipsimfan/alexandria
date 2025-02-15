use crate::Adapter;

use std::ptr::null_mut;
use win32::{
    dxgi::{CreateDXGIFactory, IDXGIFactory},
    strsafe::S_OK,
    try_hresult, ComInterface, ComPtr, DXGI_ERROR_NOT_FOUND,
};

impl Adapter {
    /// Enumerate the adapters on the system
    pub fn enumerate() -> Result<Vec<Self>, win32::Error> {
        // Create the factory
        let mut factory = ComPtr::<IDXGIFactory>::new_in(|factory| {
            try_hresult!(CreateDXGIFactory(&IDXGIFactory::IID, factory.cast()))
        })?;

        let mut adapters = Vec::new();
        loop {
            let mut adapter = null_mut();
            let result = factory.enum_adapters(adapters.len() as _, &mut adapter);
            if result == S_OK {
                adapters.push(Adapter::new(ComPtr::new(adapter))?);
            } else if result == DXGI_ERROR_NOT_FOUND {
                break;
            } else {
                return Err(win32::Error::new(result));
            }
        }

        Ok(adapters)
    }
}
