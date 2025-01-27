//! Graphics library wrapping low-level graphics APIs

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use std::ptr::null_mut;
use win32::{
    dxgi::{CreateDXGIFactory, IDXGIFactory},
    strsafe::S_OK,
    try_hresult, COMInterface, COMPtr, DXGI_ERROR_NOT_FOUND,
};

/// Enumerates all the available adapters on the system
pub fn enumerate_adapters() -> Result<(), win32::Error> {
    // Create the factory
    let mut factory = null_mut();
    try_hresult!(CreateDXGIFactory(&IDXGIFactory::IID, &mut factory)).unwrap();
    let mut factory = COMPtr::new(factory as *mut IDXGIFactory);

    let mut i = 0;
    loop {
        let mut adapter = null_mut();
        let result = factory.enum_adapters(i, &mut adapter);
        if result == S_OK {
            println!("Adapter found!");
        } else if result == DXGI_ERROR_NOT_FOUND {
            break;
        } else {
            panic!("Invalid result: {}", result);
        }

        i += 1;
    }

    Ok(())
}
