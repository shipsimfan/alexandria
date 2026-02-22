use crate::{Error, Result, shared_object::SharedObjectInner};
use linux::dlfcn::{RTLD_LOCAL, RTLD_NOW, dlerror, dlopen};
use std::{
    ffi::{CStr, CString},
    path::Path,
    ptr::null_mut,
};

impl SharedObjectInner {
    /// Open a new shared object at `path`
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SharedObjectInner> {
        let path = path.as_ref();
        let path_c = CString::new(path.to_string_lossy().as_bytes()).unwrap();

        let handle = unsafe { dlopen(path_c.as_ptr(), RTLD_LOCAL | RTLD_NOW) };
        if handle == null_mut() {
            let error = unsafe { dlerror() };
            return Err(Error::new_with(
                format!("unable to open \"{}\"", path.display()),
                unsafe { CStr::from_ptr(error) }
                    .to_string_lossy()
                    .to_string(),
            ));
        }

        Ok(SharedObjectInner {
            handle,
            name: path.to_path_buf(),
        })
    }
}
