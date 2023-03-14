pub enum ErrorKind {
    // Creation Errors
    CreateInstanceDebug,
    CreateInstanceFactory,
    CreateWindowClass,
    CreateAdapter,
    CreateDisplay,
    CreateWindow,
    CreateD3D12Device,
    CreateD3D12Debug,
    CreateCommandQueue,
    CreateSwapChain,
    CreateHeap,

    // Enumeration Errors
    EnumAdapter,
    EnumDisplay,
    EnumAvailableResolution,
    EnumBackBuffer,
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Creation Errors
                ErrorKind::CreateInstanceDebug => "Unable to create the instance debugger",
                ErrorKind::CreateInstanceFactory => "Unable to create the instance factory",
                ErrorKind::CreateWindowClass => "Unable to create the window class",
                ErrorKind::CreateAdapter => "Unable to create an adapter",
                ErrorKind::CreateDisplay => "Unable to create a display",
                ErrorKind::CreateWindow => "Unable to create the window",
                ErrorKind::CreateD3D12Device => "Unable to create the D3D12 device",
                ErrorKind::CreateD3D12Debug => "Unable to create the D3D12 debugger",
                ErrorKind::CreateCommandQueue => "Unable to create the command queue",
                ErrorKind::CreateSwapChain => "Unable to create the swap chain",
                ErrorKind::CreateHeap => "Unable to create a heap",

                // Enumeration Errors
                ErrorKind::EnumAdapter => "Unable to enumerate an adapter",
                ErrorKind::EnumDisplay => "Unable to enumerate a display",
                ErrorKind::EnumAvailableResolution => "Unable to enumerate an available resolution",
                ErrorKind::EnumBackBuffer => "Unable to enumerate a back buffer",
            }
        )
    }
}
