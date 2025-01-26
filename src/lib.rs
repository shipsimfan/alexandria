//! Graphics library wrapping low-level graphics APIs

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use std::ptr::null_mut;
use win32::{
    dxgi::{CreateDXGIFactory, IDXGIFactory},
    try_hresult, COMInterface, COMPtr,
};

/// Enumerates all the available adapters on the system
pub fn enumerate_adapters() -> Result<(), win32::Error> {
    // Create the factory
    let mut factory = null_mut();
    try_hresult!(CreateDXGIFactory(&IDXGIFactory::IID, &mut factory)).unwrap();
    let factory = COMPtr::new(factory as *mut IDXGIFactory);

    drop(factory);

    Ok(())
}
