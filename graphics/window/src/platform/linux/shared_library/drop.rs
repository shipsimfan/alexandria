use crate::platform::linux::SharedLibrary;
use linux::dlfcn::{dlclose, dlerror};
use std::ffi::CStr;

impl Drop for SharedLibrary {
    fn drop(&mut self) {
        if unsafe { dlclose(self.handle) } != 0 {
            let error = unsafe { CStr::from_ptr(dlerror()) };
            panic!(
                "unable to close \"{}\" - {}",
                self.name,
                error.to_string_lossy()
            )
        }
    }
}
