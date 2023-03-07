use crate::{Display, Result};
use common::Adapter as CommonAdapter;

#[cfg(target_os = "windows")]
type InnerAdapter = dx12::Adapter;

pub struct Adapter {
    inner: InnerAdapter,
}

pub struct DisplayIter<'a> {
    inner: <InnerAdapter as CommonAdapter>::DisplayIter<'a>,
}

impl Adapter {
    pub(crate) fn from_inner(inner: InnerAdapter) -> Self {
        Adapter { inner }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.inner.name()
    }

    #[inline]
    pub fn enum_displays<'a>(&'a mut self) -> Result<DisplayIter> {
        self.inner
            .enum_displays()
            .map(|inner| DisplayIter { inner })
    }

    #[inline]
    pub fn default_display(&mut self) -> Result<Display> {
        self.inner
            .default_display()
            .map(|inner| Display::from_inner(inner))
    }
}

impl<'a> Iterator for DisplayIter<'a> {
    type Item = Result<Display>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|result| result.map(|inner| Display::from_inner(inner)))
    }
}
