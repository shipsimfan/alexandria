use crate::{Display, Result};
use win32::{DXGIAdapter, Interface};

pub struct Adapter {
    adapter: win32::IDXGIAdapter4,

    name: String,
}

pub struct DisplayIter<'a> {
    adapter: &'a mut Adapter,
    index: usize,
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
        Ok(DisplayIter {
            adapter: self,
            index: 0,
        })
    }

    pub fn default_display(&mut self) -> Result<Display> {
        self.enum_display(0).map(|display| display.unwrap())
    }

    pub(self) fn enum_display(&mut self, display: u32) -> Result<Option<Display>> {
        match self.adapter.enum_outputs(display) {
            Ok(display) => match display {
                Some(display) => Ok(Some(Display::new(display)?)),
                None => Ok(None),
            },
            Err(error) => Err(error),
        }
    }
}

impl<'a> Iterator for DisplayIter<'a> {
    type Item = Result<Display>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.adapter.enum_display(self.index as u32) {
            Ok(display) => match display {
                Some(display) => {
                    self.index += 1;
                    Some(Ok(display))
                }
                None => None,
            },
            Err(error) => Some(Err(error)),
        }
    }
}
