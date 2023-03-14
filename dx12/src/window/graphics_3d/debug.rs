use crate::map_raw_error;

use super::device::Device;
use crate::Result;
use common::{DebugMessage, DebugMessageLevel};
use std::sync::{Arc, Mutex};

pub(crate) struct Debug {
    info_queue: win32::ID3D12InfoQueue,
    current_message: u64,
}

impl Debug {
    pub(super) fn new(device: &mut Device) -> Result<Arc<Mutex<Self>>> {
        let info_queue = map_raw_error!(device.get_debug(), CreateD3D12Debug)?;

        Ok(Arc::new(Mutex::new(Debug {
            info_queue,
            current_message: 0,
        })))
    }

    pub(crate) fn pop_message(&mut self) -> Option<DebugMessage> {
        let num_messages = self
            .info_queue
            .get_num_stored_messages_allowed_by_retrieval_filter();
        if num_messages <= self.current_message {
            return None;
        }

        let message = self.info_queue.get_message(self.current_message).unwrap();

        self.current_message += 1;

        Some(DebugMessage::new(
            message.description().to_owned(),
            match message.severity() {
                win32::D3D12MessageSeverity::Corruption => DebugMessageLevel::Fatal,
                win32::D3D12MessageSeverity::Error => DebugMessageLevel::Error,
                win32::D3D12MessageSeverity::Warning => DebugMessageLevel::Warning,
                win32::D3D12MessageSeverity::Info | win32::D3D12MessageSeverity::Message => {
                    DebugMessageLevel::Info
                }
            },
        ))
    }
}
