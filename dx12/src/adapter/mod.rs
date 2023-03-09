use crate::{Display, Result};
use win32::Interface;

mod display_iter;

pub use display_iter::*;

pub struct Adapter {
    adapter: win32::IDXGIAdapter4,

    name: String,
}

impl Adapter {
    pub(crate) fn new(mut adapter: win32::IDXGIAdapter1) -> Result<Self> {
        let mut adapter: win32::IDXGIAdapter4 = adapter.query_interface()?;

        let desc = adapter.get_desc3()?;

        let name = String::from_utf16(desc.description()).unwrap();

        Ok(Adapter { adapter, name })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn enum_displays<'a>(&'a mut self) -> Result<DisplayIter<'a>> {
        Ok(DisplayIter::new(&mut self.adapter))
    }

    pub fn default_display(&mut self) -> Result<Display> {
        self.enum_displays()?.next().unwrap()
    }
}
