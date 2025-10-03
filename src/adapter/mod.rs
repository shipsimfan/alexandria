use win32::{dxgi::IDXGIAdapter1, ComPtr};

mod enumerate;
mod get;
mod new;

/// A graphics adapter
pub struct Adapter {
    /// The underlying adapter
    #[allow(unused)]
    adapter: ComPtr<IDXGIAdapter1>,

    /// The name of the adapter
    name: String,

    /// The amount of dedicated video memory this adapter has
    video_memory: u64,

    /// Is this a software adapter?
    is_software: bool,
}
