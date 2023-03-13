use crate::Result;
use common::{DebugMessage, DebugMessageLevel};
use win32::{D3D12Debug, DXGIDebug};

pub(super) struct Debug {
    info_queue: win32::IDXGIInfoQueue,
    current_message: u64,
}

impl Debug {
    pub(super) fn new() -> Result<Self> {
        let mut debug = win32::dxgi_get_debug_interface1::<win32::IDXGIDebug1>()?;
        debug.report_live_objects(win32::DXGIDebugID::All, win32::DXGIDebugRLOFlag::Summary)?;

        let info_queue = win32::dxgi_get_debug_interface1::<win32::IDXGIInfoQueue>()?;

        let mut debug = win32::d3d12_get_debug_interface::<win32::ID3D12Debug3>()?;
        debug.enable_debug_layer();

        Ok(Debug {
            info_queue,
            current_message: 0,
        })
    }

    pub(super) fn pop_message(&mut self) -> Result<Option<DebugMessage>> {
        let num_messages = self
            .info_queue
            .get_num_stored_messages_allowed_by_retrieval_filters(win32::DXGIDebugID::All);
        if num_messages <= self.current_message {
            return Ok(None);
        }

        println!("{} ({})", num_messages, self.current_message);

        let message = self
            .info_queue
            .get_message(win32::DXGIDebugID::All, self.current_message)?;

        self.current_message += 1;

        Ok(Some(DebugMessage::new(
            message.description().to_owned(),
            match message.severity() {
                win32::DXGIInfoQueueMessageSeverity::Corruption => DebugMessageLevel::Fatal,
                win32::DXGIInfoQueueMessageSeverity::Error => DebugMessageLevel::Error,
                win32::DXGIInfoQueueMessageSeverity::Warning => DebugMessageLevel::Warning,
                win32::DXGIInfoQueueMessageSeverity::Info
                | win32::DXGIInfoQueueMessageSeverity::Message => DebugMessageLevel::Info,
            },
        )))
    }
}
