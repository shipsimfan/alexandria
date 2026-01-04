use crate::platform::linux::SharedLibrary;
use linux::dlfcn::{RTLD_LOCAL, RTLD_NOW, dlopen};
use std::{ffi::CString, ptr::null_mut, str::FromStr};

impl SharedLibrary {
    /// Try to open the [`SharedLibrary`] named `name`
    pub fn try_open(name: &'static str) -> Option<SharedLibrary> {
        let c_name = CString::from_str(name).unwrap();
        let handle = unsafe { dlopen(c_name.as_ptr(), RTLD_NOW | RTLD_LOCAL) };
        if handle == null_mut() {
            None
        } else {
            Some(SharedLibrary { handle, name })
        }
    }
}
