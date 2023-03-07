use common::Display as CommonDisplay;

#[cfg(target_os = "windows")]
type InnerDisplay = dx12::Display;

pub struct Display {
    inner: InnerDisplay,
}

impl Display {
    pub(crate) fn from_inner(inner: InnerDisplay) -> Self {
        Display { inner }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.inner.name()
    }
}
