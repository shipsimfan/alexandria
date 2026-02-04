use crate::shared_object::SharedObjectInner;
use linux::dlfcn::dlsym;
use std::ffi::{CStr, c_void};

impl SharedObjectInner {
    /// Load the symbol named `symbol` from this shared object
    pub(in crate::shared_object) unsafe fn load_raw_symbol(&self, symbol: &CStr) -> *mut c_void {
        unsafe { dlsym(self.handle, symbol.as_ptr()) }
    }
}
