use crate::platform::linux::wayland::{WaylandLibrary, WlDisplay};
use std::{
    cell::RefCell,
    ptr::{null, null_mut},
    rc::Rc,
};

impl WlDisplay {
    /// Attempt to connect to Wayland
    pub fn try_connect() -> Option<Rc<WlDisplay>> {
        let library = WaylandLibrary::try_open()?;

        let handle = unsafe { (library.f.display_connect)(null()) };
        if handle == null_mut() {
            None
        } else {
            Some(Rc::new(WlDisplay {
                handle: RefCell::new(handle),
                library,
            }))
        }
    }
}
