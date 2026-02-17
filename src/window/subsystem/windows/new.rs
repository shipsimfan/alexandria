use crate::{
    Error, Result,
    window::{display::DisplayInner, subsystem::WindowSubsystemInner},
};
use win32::{
    ComInterface, ComPtr, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
    SetProcessDpiAwarenessContext,
    dxgi::{CreateDXGIFactory, IDXGIFactory},
    try_get_last_error, try_hresult,
};

impl WindowSubsystemInner {
    /// Create a new [`WindowSubsystemInner`]
    pub(in crate::window::subsystem) fn new() -> Result<WindowSubsystemInner> {
        // Set the DPI awareness
        try_get_last_error!(SetProcessDpiAwarenessContext(
            DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2
        ))
        .map_err(|os| Error::new_with("unable to set DPI awareness", os))?;

        // Create DXGI factory
        let mut dxgi_factory = ComPtr::<IDXGIFactory>::new_in(|factory| {
            try_hresult!(CreateDXGIFactory(&IDXGIFactory::IID, factory.cast()))
        })
        .map_err(|os| Error::new_with("unable to create a DXGI factory", os))?;

        // Enumerate the available displays
        let displays = DisplayInner::enumerate(&mut dxgi_factory)?
            .into_iter()
            .collect();

        Ok(WindowSubsystemInner {
            displays,
            dxgi_factory,
        })
    }
}
