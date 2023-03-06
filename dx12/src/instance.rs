use crate::Adapter;
use win32::DXGIFactory1;

pub struct Instance {
    dxgi_factory: win32::IDXGIFactory7,
}

pub struct AdapterIter<'a> {
    instance: &'a mut Instance,
    index: usize,
}

impl Instance {
    pub(self) fn enum_adapter(&mut self, adapter: u32) -> Result<Option<Adapter>, crate::Error> {
        match self.dxgi_factory.enum_adapters1(adapter) {
            Ok(adapter) => match adapter {
                Some(adapter) => Ok(Some(Adapter::new(adapter)?)),
                None => Ok(None),
            },
            Err(error) => Err(error),
        }
    }
}

impl common::Instance for Instance {
    type Error = crate::Error;

    type Adapter = crate::Adapter;
    type AdapterIter<'a> = AdapterIter<'a>;

    fn new() -> Result<Self, Self::Error> {
        #[cfg(debug_assertions)]
        let flags = &[win32::DXGICreateFactoryFlag::Debug];
        #[cfg(not(debug_assertions))]
        let flags = &[];

        let dxgi_factory = win32::create_factory2(flags)?;

        Ok(Instance { dxgi_factory })
    }

    fn enum_adapters<'a>(&'a mut self) -> Result<Self::AdapterIter<'a>, Self::Error> {
        Ok(AdapterIter {
            instance: self,
            index: 0,
        })
    }

    fn default_adapter(&mut self) -> Result<Self::Adapter, Self::Error> {
        self.enum_adapter(0).map(|adapter| adapter.unwrap())
    }
}

impl<'a> Iterator for AdapterIter<'a> {
    type Item = Result<crate::Adapter, crate::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.instance.enum_adapter(self.index as u32) {
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
