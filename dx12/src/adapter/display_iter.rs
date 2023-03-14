use crate::{instance::Debug, instance_error, Display, Result};
use std::sync::{Arc, Mutex};
use win32::DXGIAdapter;

pub struct DisplayIter<'a> {
    adapter: &'a mut win32::IDXGIAdapter4,
    index: usize,

    debug: Option<&'a Arc<Mutex<Debug>>>,
}

impl<'a> DisplayIter<'a> {
    pub(super) fn new(
        adapter: &'a mut win32::IDXGIAdapter4,
        debug: Option<&'a Arc<Mutex<Debug>>>,
    ) -> Self {
        DisplayIter {
            adapter,
            index: 0,
            debug,
        }
    }

    fn enum_display(&mut self, display: u32) -> Result<Option<Display>> {
        match self.adapter.enum_outputs(display) {
            Ok(display) => match display {
                Some(display) => Ok(Some(Display::new(
                    display,
                    self.debug.clone().map(|debug| debug.to_owned()),
                )?)),
                None => Ok(None),
            },
            Err(error) => Err(instance_error!(EnumDisplay, error, self.debug)),
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
