use crate::{
    Error, Result,
    window::subsystem::linux::{WlDisplay, wayland::WaylandLibrary},
};
use std::{
    cell::RefCell,
    ptr::{null, null_mut},
    rc::Rc,
};

impl WlDisplay {
    /// Attempt to connect to Wayland
    pub fn try_connect() -> Result<Rc<WlDisplay>> {
        let library = WaylandLibrary::try_open()?;

        let handle = unsafe { (library.f.display_connect)(null()) };
        if handle == null_mut() {
            Err(Error::new("unable to connect to Wayland"))
        } else {
            Ok(Rc::new(WlDisplay {
                handle: RefCell::new(handle),
                library,
            }))
        }
    }
}
