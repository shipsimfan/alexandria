use crate::{graphics::render_context::InfoQueue, Error, Result};
use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    ffi::CStr,
};
use win32::{
    d3d11sdklayers::{D3D11_MESSAGE, D3D11_MESSAGE_SEVERITY},
    try_hresult,
};

const MAX_MESSAGE_LENGTH: usize = 256;
const MESSAGE_BUFFER_LAYOUT: Layout = unsafe {
    Layout::from_size_align_unchecked(
        std::mem::size_of::<D3D11_MESSAGE>() + MAX_MESSAGE_LENGTH,
        std::mem::align_of::<D3D11_MESSAGE>(),
    )
};

impl InfoQueue {
    /// Empty all messages from the queue
    pub fn empty_queue(&mut self) -> Result<()> {
        let message_ptr = unsafe { alloc_zeroed(MESSAGE_BUFFER_LAYOUT) };
        let message = unsafe { &mut *(message_ptr as *mut D3D11_MESSAGE) };

        let count = self.handle.get_num_stored_messages();
        for i in 0..count {
            let mut size = MESSAGE_BUFFER_LAYOUT.size() as _;
            try_hresult!(self.handle.get_message(i, message, &mut size)).map_err(|error| {
                unsafe { dealloc(message_ptr, MESSAGE_BUFFER_LAYOUT) };
                Error::new_os("unable to get info queue message", error)
            })?;

            let description = unsafe { CStr::from_ptr(message.description) };
            let severity = match message.severity {
                D3D11_MESSAGE_SEVERITY::Corruption => "CORRUPTION",
                D3D11_MESSAGE_SEVERITY::Error => "ERROR",
                D3D11_MESSAGE_SEVERITY::Warning => "WARNING",
                D3D11_MESSAGE_SEVERITY::Info => "INFO",
                D3D11_MESSAGE_SEVERITY::Message => "DEBUG",
                _ => "UNKNOWN",
            };

            println!("{}: {}", severity, description.to_string_lossy());
        }

        self.handle.clear_stored_messages();

        unsafe { dealloc(message_ptr, MESSAGE_BUFFER_LAYOUT) };

        Ok(())
    }
}
