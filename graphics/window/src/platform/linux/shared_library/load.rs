use crate::platform::linux::SharedLibrary;
use linux::dlfcn::{dlerror, dlsym};
use std::{
    ffi::{CStr, c_void},
    ptr::null_mut,
};

impl SharedLibrary {
    /// Load a symbol from the library
    pub fn load(&mut self, symbol: &CStr) -> *mut c_void {
        self.try_load(symbol).unwrap_or_else(|| {
            let error = unsafe { CStr::from_ptr(dlerror()) };
            panic!(
                "unable to load \"{}\" from \"{}\" - {}",
                self.name,
                symbol.to_string_lossy(),
                error.to_string_lossy()
            );
        })
    }

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
