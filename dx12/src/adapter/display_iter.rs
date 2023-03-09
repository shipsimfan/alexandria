use crate::{Display, Result};
use win32::DXGIAdapter;

pub struct DisplayIter<'a> {
    adapter: &'a mut win32::IDXGIAdapter4,
    index: usize,
}

impl<'a> DisplayIter<'a> {
    pub(super) fn new(adapter: &'a mut win32::IDXGIAdapter4) -> Self {
        DisplayIter { adapter, index: 0 }
    }

    fn enum_display(&mut self, display: u32) -> Result<Option<Display>> {
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
        match self.enum_display(self.index as u32) {
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
