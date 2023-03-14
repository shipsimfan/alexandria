use super::Debug;
use crate::{instance_error, Adapter, Result};
use std::sync::{Arc, Mutex};
use win32::DXGIFactory1;

pub struct AdapterIter<'a> {
    factory: &'a mut win32::IDXGIFactory7,
    index: usize,

    debug: Option<&'a Arc<Mutex<Debug>>>,
}

impl<'a> AdapterIter<'a> {
    pub(super) fn new(
        factory: &'a mut win32::IDXGIFactory7,
        debug: Option<&'a Arc<Mutex<Debug>>>,
    ) -> Self {
        AdapterIter {
            factory,
            index: 0,
            debug,
        }
    }

    fn enum_adapter(&mut self, adapter: u32) -> Result<Option<Adapter>> {
        match self.factory.enum_adapters1(adapter) {
            Ok(adapter) => match adapter {
                Some(adapter) => Ok(Some(Adapter::new(
                    adapter,
                    self.debug.clone().map(|debug| debug.to_owned()),
                )?)),
                None => Ok(None),
            },
            Err(error) => Err(instance_error!(EnumAdapter, error, self.debug)),
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
