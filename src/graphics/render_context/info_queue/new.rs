use crate::{graphics::render_context::InfoQueue, Error, Result};
use win32::{
    d3d11::ID3D11Device,
    d3d11sdklayers::{
        ID3D11InfoQueue, D3D11_INFO_QUEUE_FILTER, D3D11_INFO_QUEUE_FILTER_DESC,
        D3D11_MESSAGE_SEVERITY,
    },
    try_hresult, ComPtr,
};

impl InfoQueue {
    /// Create a new [`InfoQueue`]
    pub fn new(device: &mut ComPtr<ID3D11Device>) -> Result<Self> {
        let mut handle: ComPtr<ID3D11InfoQueue> = device
            .query_interface()
            .map_err(|error| Error::new_os("unable to get device info queue", error))?;

        // Allow all messages with no limit
        try_hresult!(handle.set_message_count_limit(-1 as _))
            .map_err(|error| Error::new_os("unable to clear info queue message limit", error))?;
        handle.clear_retrieval_filter();
        handle.clear_storage_filter();

        // Block "INFO" messages
        handle.add_storage_filter_entries(&mut D3D11_INFO_QUEUE_FILTER {
            deny_list: D3D11_INFO_QUEUE_FILTER_DESC {
                num_severities: 1,
                severity_list: &mut D3D11_MESSAGE_SEVERITY::Info,
                ..Default::default()
            },
            ..Default::default()
        });

        Ok(InfoQueue { handle })
    }
}
