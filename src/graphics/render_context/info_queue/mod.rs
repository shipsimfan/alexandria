use win32::{d3d11sdklayers::ID3D11InfoQueue, ComPtr};

mod empty_queue;
mod new;

/// The info queue of messages from the system graphics API
pub(in crate::graphics::render_context) struct InfoQueue {
    /// A handle to the info queue
    handle: ComPtr<ID3D11InfoQueue>,
}
