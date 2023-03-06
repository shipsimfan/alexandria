use common::Adapter as CommonAdapter;

#[cfg(target_os = "windows")]
type InnerAdapter = dx12::Adapter;

pub struct Adapter {
    inner: InnerAdapter,
}

impl Adapter {
    pub(crate) fn from_inner(inner: InnerAdapter) -> Self {
        Adapter { inner }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.inner.name()
    }
}
