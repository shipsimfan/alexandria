use crate::{Error, Result, shared_object::SharedObjectInner};
use std::path::Path;
use win32::{LoadLibrary, try_get_last_error};

impl SharedObjectInner {
    /// Open a new shared object at `path`
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SharedObjectInner> {
        let path = path.as_ref();
        let path_utf16: Vec<_> = path.to_string_lossy().encode_utf16().chain([0]).collect();

        let handle = try_get_last_error!(LoadLibrary(path_utf16.as_ptr()))
            .map_err(|os| Error::new_with(format!("unable to open \"{}\"", path.display()), os))?;

        Ok(SharedObjectInner {
            handle,
            name: path.to_path_buf(),
        })
    }
}
