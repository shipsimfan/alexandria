use crate::{
    graphics::render_context::InfoQueue, Error, GraphicsApiLogSeverity, LogCallbacks, Result,
};
use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    borrow::Cow,
    ffi::CStr,
    ptr::null_mut,
};
use win32::{
    d3d11sdklayers::{D3D11_MESSAGE, D3D11_MESSAGE_SEVERITY},
    try_hresult,
};

impl InfoQueue {
    /// Empty all messages from the queue
    pub fn empty_queue(&mut self, log_callbacks: &mut dyn LogCallbacks) -> Result<()> {
        let count = self.handle.get_num_stored_messages();
        for i in 0..count {
            // Get the message size
            let mut size = 0;
            try_hresult!(self.handle.get_message(i, null_mut(), &mut size))
                .map_err(|error| Error::new_os("unable to get info queue message size", error))?;

            // Allocate space for the message
            let message_layout = Layout::from_size_align(
                std::mem::size_of::<D3D11_MESSAGE>() + size as usize,
                std::mem::align_of::<D3D11_MESSAGE>(),
            )
            .unwrap();
            let message_ptr = unsafe { alloc_zeroed(message_layout) };
            let message = unsafe { &mut *(message_ptr as *mut D3D11_MESSAGE) };

            // Get the message
            try_hresult!(self.handle.get_message(i, message, &mut size)).map_err(|error| {
                unsafe { dealloc(message_ptr, message_layout) };
                Error::new_os("unable to get info queue message", error)
            })?;

            // Extract the information from the message
            let description = match unsafe { CStr::from_ptr(message.description) }.to_string_lossy()
            {
                Cow::Owned(owned) => owned,
                Cow::Borrowed(borrowed) => borrowed.to_string(),
            };
            let severity = match message.severity {
                D3D11_MESSAGE_SEVERITY::Corruption | D3D11_MESSAGE_SEVERITY::Error => {
                    GraphicsApiLogSeverity::Error
                }
                D3D11_MESSAGE_SEVERITY::Warning => GraphicsApiLogSeverity::Warning,
                _ => GraphicsApiLogSeverity::Info,
            };

            unsafe { dealloc(message_ptr, message_layout) };

            log_callbacks.on_graphics_api_log(severity, description);
        }

        self.handle.clear_stored_messages();
        Ok(())
    }
}
