use crate::{Adapter, Result};
use std::{cmp::Ordering, ptr::null_mut};
use win32::{
    dxgi::{CreateDXGIFactory1, IDXGIFactory1},
    strsafe::S_OK,
    try_hresult, ComInterface, ComPtr, DXGI_ERROR_NOT_FOUND,
};

impl Adapter {
    /// Enumerate the adapters on the system
    pub fn enumerate() -> Result<Vec<Self>> {
        // Create the factory
        let mut factory = ComPtr::<IDXGIFactory1>::new_in(|factory| {
            try_hresult!(CreateDXGIFactory1(&IDXGIFactory1::IID, factory.cast()))
        })?;

        // Enumerate the adapters
        let mut adapters = Vec::new();
        loop {
            let mut adapter = null_mut();
            let result = factory.enum_adapters1(adapters.len() as _, &mut adapter);
            if result == S_OK {
                adapters.push(Adapter::new(ComPtr::new(adapter))?);
            } else if result == DXGI_ERROR_NOT_FOUND {
                break;
            } else {
                return Err(win32::Error::new(result).into());
            }
        }

        // Sort adapters into recommended order:
        //  1. Hardware > Software
        //  2. More Video Memory
        //  3. Alphabetical
        adapters.sort_by(|a, b| match a.is_software.cmp(&b.is_software) {
            Ordering::Equal => match b.video_memory.cmp(&a.video_memory) {
                Ordering::Equal => a.name.cmp(&b.name),
                ordering => ordering,
            },
            ordering => ordering,
        });

        Ok(adapters)
    }
}
