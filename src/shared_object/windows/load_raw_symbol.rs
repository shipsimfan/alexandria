use crate::shared_object::SharedObjectInner;
use std::{
    ffi::{CStr, c_void},
    ptr::null_mut,
};
use win32::GetProcAddress;

impl SharedObjectInner {
    /// Load the symbol named `symbol` from this shared object
    pub unsafe fn load_raw_symbol(&self, symbol: &CStr) -> *mut c_void {
        match unsafe { GetProcAddress(self.handle, symbol.as_ptr()) } {
            Some(ptr) => unsafe { std::mem::transmute(ptr) },
            None => null_mut(),
        }
    }
}
