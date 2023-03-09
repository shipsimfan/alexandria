use crate::{Adapter, Result};
use win32::DXGIFactory1;

pub struct AdapterIter<'a> {
    factory: &'a mut win32::IDXGIFactory7,
    index: usize,
}

impl<'a> AdapterIter<'a> {
    pub(super) fn new(factory: &'a mut win32::IDXGIFactory7) -> Self {
        AdapterIter { factory, index: 0 }
    }

    fn enum_adapter(&mut self, adapter: u32) -> Result<Option<Adapter>> {
        match self.factory.enum_adapters1(adapter) {
            Ok(adapter) => match adapter {
                Some(adapter) => Ok(Some(Adapter::new(adapter)?)),
                None => Ok(None),
            },
            Err(error) => Err(error),
        }
    }
}

impl<'a> Iterator for AdapterIter<'a> {
    type Item = Result<Adapter>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.enum_adapter(self.index as u32) {
            Ok(adapter) => match adapter {
                Some(adapter) => {
                    self.index += 1;
                    Some(Ok(adapter))
                }
                None => None,
            },
            Err(error) => Some(Err(error)),
        }
    }
}
