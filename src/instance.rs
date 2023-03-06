use crate::{Adapter, Result};
use common::Instance as CommonInstance;

#[cfg(target_os = "windows")]
type InnerInstance = dx12::Instance;

pub struct Instance {
    inner: InnerInstance,
}

pub struct AdapterIter<'a> {
    inner: <InnerInstance as CommonInstance>::AdapterIter<'a>,
}

impl Instance {
    #[inline]
    pub fn new() -> Result<Self> {
        InnerInstance::new().map(|inner| Instance { inner })
    }

    #[inline]
    pub fn enum_adapters<'a>(&'a mut self) -> Result<AdapterIter> {
        self.inner
            .enum_adapters()
            .map(|inner| AdapterIter { inner })
    }

    #[inline]
    pub fn default_adapter(&mut self) -> Result<Adapter> {
        self.inner
            .default_adapter()
            .map(|inner| Adapter::from_inner(inner))
    }
}

impl<'a> Iterator for AdapterIter<'a> {
    type Item = Result<Adapter>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|result| result.map(|inner| Adapter::from_inner(inner)))
    }
}
