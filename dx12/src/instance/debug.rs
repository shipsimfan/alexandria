use common::{DebugMessage, DebugMessageLevel};
use std::sync::{Arc, Mutex};
use win32::DXGIDebug;

pub(crate) struct Debug {
    //info_queue: win32::IDXGIInfoQueue,
    current_message: u64,
}

impl Debug {
    pub(super) fn new() -> Result<Arc<Mutex<Self>>, win32::Error> {
        let mut debug = win32::dxgi_get_debug_interface1::<win32::IDXGIDebug1>()?;
        unsafe { debug.as_mut() }
            .report_live_objects(win32::DXGI_DEBUG_ALL, win32::DXGIDebugRLOFlag::Summary)?;

        //let info_queue = win32::dxgi_get_debug_interface1::<win32::IDXGIInfoQueue>()?;

        /*
        let mut debug = win32::d3d12_get_debug_interface::<win32::ID3D12Debug3>()?;
        debug.enable_debug_layer();
        */

        Ok(Arc::new(Mutex::new(Debug {
            //info_queue,
            current_message: 0,
        })))
    }

    /*
    pub(crate) fn pop_message(&mut self) -> Option<DebugMessage> {
        let num_messages = self
            .info_queue
            .get_num_stored_messages_allowed_by_retrieval_filters(win32::DXGIDebugID::All);
        if num_messages <= self.current_message {
            return None;
        }

        let message = self
            .info_queue
            .get_message(win32::DXGIDebugID::All, self.current_message)
            .unwrap();

        self.current_message += 1;

        Some(DebugMessage::new(
            message.description().to_owned(),
            match message.severity() {
                win32::DXGIInfoQueueMessageSeverity::Corruption => DebugMessageLevel::Fatal,
                win32::DXGIInfoQueueMessageSeverity::Error => DebugMessageLevel::Error,
                win32::DXGIInfoQueueMessageSeverity::Warning => DebugMessageLevel::Warning,
                win32::DXGIInfoQueueMessageSeverity::Info
                | win32::DXGIInfoQueueMessageSeverity::Message => DebugMessageLevel::Info,
            },
        ))
    }
    */
}
