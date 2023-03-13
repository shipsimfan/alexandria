use crate::Result;
use common::{DebugMessage, DebugMessageLevel};
use win32::Interface;

pub(super) struct Debug {
    info_queue: win32::ID3D12InfoQueue,
    current_message: u64,
}

impl Debug {
    pub(super) fn new(device: &mut win32::ID3D12Device) -> Result<Self> {
        let info_queue = device.query_interface()?;

        Ok(Debug {
            info_queue,
            current_message: 0,
        })
    }

    pub(super) fn pop_message(&mut self) -> Result<Option<DebugMessage>> {
        let num_messages = self
            .info_queue
            .get_num_stored_messages_allowed_by_retrieval_filter();
        if num_messages <= self.current_message {
            return Ok(None);
        }

        let message = self.info_queue.get_message(self.current_message)?;

        self.current_message += 1;

        Ok(Some(DebugMessage::new(
            message.description().to_owned(),
            match message.severity() {
                win32::D3D12MessageSeverity::Corruption => DebugMessageLevel::Fatal,
                win32::D3D12MessageSeverity::Error => DebugMessageLevel::Error,
                win32::D3D12MessageSeverity::Warning => DebugMessageLevel::Warning,
                win32::D3D12MessageSeverity::Info | win32::D3D12MessageSeverity::Message => {
                    DebugMessageLevel::Info
                }
            },
        )))
    }
}
