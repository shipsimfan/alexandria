use crate::platform::linux::SharedLibrary;
use linux::dlfcn::dlsym;
use std::{
    ffi::{CStr, c_void},
    ptr::null_mut,
};

impl SharedLibrary {
    /// Try to load a symbol from the library
    pub fn try_load(&mut self, symbol: &CStr) -> Option<*mut c_void> {
        let ptr = unsafe { dlsym(self.handle, symbol.as_ptr()) };
        if ptr == null_mut() { None } else { Some(ptr) }
    }
}

macro_rules! try_load_function {
    ($library: expr, $name: expr) => {
        $library
            .try_load($name)
            .map(|f| unsafe { std::mem::transmute(f) })
    };
}
pub(in crate::platform::linux) use try_load_function;
