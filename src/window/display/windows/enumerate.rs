use crate::{
    Error, Result,
    window::display::{DisplayInner, windows::DisplayConfig},
};
use std::ptr::null_mut;
use win32::{
    ComPtr, DXGI_ERROR_NOT_FOUND, S_OK,
    dxgi::{IDXGIAdapter, IDXGIFactory, IDXGIOutput},
};

/// Get the output at `index` on `adapter`
fn get_output(index: u32, adapter: &mut IDXGIAdapter) -> Result<Option<ComPtr<IDXGIOutput>>> {
    let mut output = null_mut();
    match unsafe { adapter.enum_outputs(index, &mut output) } {
        S_OK => Ok(Some(ComPtr::new(output))),
        DXGI_ERROR_NOT_FOUND => Ok(None),
        error => Err(Error::new_with(
            "unable to enumerate DXGI outputs",
            win32::Error::new(error),
        )),
    }
}

/// Enumerate the DXGI outputs on `adapter`
fn enumerate_outputs<UserEvent>(
    adapter: &mut IDXGIAdapter,
    display_configs: &[DisplayConfig],
    displays: &mut Vec<DisplayInner<UserEvent>>,
) -> Result<()> {
    let mut j = 0;
    while let Some(mut output) = get_output(j, adapter)? {
        j += 1;

        if let Some(display) = DisplayInner::new(&mut output, &display_configs)? {
            displays.push(display);
        }
    }
    Ok(())
}

/// Get the adapter at `index` in `factory`
fn get_adapter(index: u32, factory: &mut IDXGIFactory) -> Result<Option<ComPtr<IDXGIAdapter>>> {
    let mut adapter = null_mut();
    match unsafe { factory.enum_adapters(index, &mut adapter) } {
        S_OK => Ok(Some(ComPtr::new(adapter))),
        DXGI_ERROR_NOT_FOUND => Ok(None),
        error => Err(Error::new_with(
            "unable to enumerate DXGI adapters",
            win32::Error::new(error),
        )),
    }
}

/// Enumerate the DXGI adapters in `factory`
fn enumerate_adapters<UserEvent>(
    factory: &mut IDXGIFactory,
    display_configs: &[DisplayConfig],
) -> Result<Vec<DisplayInner<UserEvent>>> {
    let mut displays = Vec::new();
    let mut i = 0;
    while let Some(mut adapter) = get_adapter(i, factory)? {
        i += 1;

        enumerate_outputs(&mut adapter, display_configs, &mut displays)?;
    }
    Ok(displays)
}

impl<UserEvent> DisplayInner<UserEvent> {
    /// Enumerate all currently available displays
    pub fn enumerate(factory: &mut IDXGIFactory) -> Result<Vec<DisplayInner<UserEvent>>> {
        let display_configs = DisplayConfig::enumerate()?;
        enumerate_adapters(factory, &display_configs)
    }
}
