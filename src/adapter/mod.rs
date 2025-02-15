use win32::{dxgi::IDXGIAdapter, ComPtr};

mod enumerate;
mod get;
mod new;

/// A graphics adapter
pub struct Adapter {
    /// The underlying adapter
    #[allow(unused)]
    adapter: ComPtr<IDXGIAdapter>,

    /// The name of the adapter
    name: String,
}
