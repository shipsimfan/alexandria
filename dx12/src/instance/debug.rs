use crate::Result;
use common::{DebugMessage, DebugMessageLevel};
use win32::DXGIDebug;

pub struct Debug {
    dxgi_info_queue: win32::IDXGIInfoQueue,
    current_dxgi_message: u64,

    debugging_enabled: bool,
}

impl Debug {
    pub fn new(enable_debugging: bool) -> Result<Self> {
        if enable_debugging {
            let mut dxgi_debug = win32::dxgi_get_debug_interface1::<win32::IDXGIDebug1>()?;
            dxgi_debug
                .report_live_objects(win32::DXGIDebugID::All, win32::DXGIDebugRLOFlag::Detail)?;
        }

        let dxgi_info_queue = win32::dxgi_get_debug_interface1::<win32::IDXGIInfoQueue>()?;

        Ok(Debug {
            dxgi_info_queue,
            current_dxgi_message: 0,

            debugging_enabled: enable_debugging,
        })
    }

    pub fn pop_message(&mut self) -> Result<Option<DebugMessage>> {
        if !self.debugging_enabled {
            return Ok(None);
        }

        let num_messages = self
            .dxgi_info_queue
            .get_num_stored_messages_allowed_by_retrieval_filters(win32::DXGIDebugID::All);

        if num_messages <= self.current_dxgi_message {
            return Ok(None);
        }

        let message = self
            .dxgi_info_queue
            .get_message(win32::DXGIDebugID::All, self.current_dxgi_message)?;

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
