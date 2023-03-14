use crate::{instance::Debug, map_instance_error, Display, Result};
use std::sync::{Arc, Mutex};
use win32::Interface;

mod display_iter;

pub use display_iter::*;

pub struct Adapter {
    adapter: win32::IDXGIAdapter4,

    name: String,

    debug: Option<Arc<Mutex<Debug>>>,
}

impl Adapter {
    pub(crate) fn new(
        mut adapter: win32::IDXGIAdapter1,
        debug: Option<Arc<Mutex<Debug>>>,
    ) -> Result<Self> {
        let mut adapter: win32::IDXGIAdapter4 =
            map_instance_error!(adapter.query_interface(), CreateAdapter, debug)?;

        let desc = map_instance_error!(adapter.get_desc3(), CreateAdapter, debug)?;

        let name = String::from_utf16(desc.description()).unwrap();

        Ok(Adapter {
            adapter,
            name,
            debug,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn enum_displays<'a>(&'a mut self) -> Result<DisplayIter<'a>> {
        Ok(DisplayIter::new(&mut self.adapter, self.debug.as_ref()))
    }

    pub fn default_display(&mut self) -> Result<Display> {
        self.enum_displays()?.next().unwrap()
    }

    pub(crate) fn inner(&mut self) -> &mut win32::IDXGIAdapter4 {
        &mut self.adapter
    }
}
