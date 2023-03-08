use common::{Display as CommonDisplay, DisplayMode as CommonDisplayMode};

#[cfg(target_os = "windows")]
type InnerDisplay = dx12::Display;

type InnerDisplayMode = <InnerDisplay as CommonDisplay>::DisplayMode;
type InnerDisplayModeIter<'a> = <InnerDisplay as CommonDisplay>::DisplayModeIter<'a>;

pub struct Display {
    inner: InnerDisplay,
}

pub struct DisplayMode<'a> {
    inner: &'a InnerDisplayMode,
}

pub struct DisplayModeIter<'a> {
    inner: InnerDisplayModeIter<'a>,
}

impl Display {
    pub(crate) fn from_inner(inner: InnerDisplay) -> Self {
        Display { inner }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.inner.name()
    }

    #[inline]
    pub fn display_modes(&self) -> DisplayModeIter {
        DisplayModeIter {
            inner: self.inner.display_modes(),
        }
    }
}

impl<'a> DisplayMode<'a> {
    #[inline]
    pub fn width(&self) -> usize {
        self.inner.width()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.inner.height()
    }

    #[inline]
    pub fn refresh_rate(&self) -> f32 {
        self.inner.refresh_rate()
    }
}

impl<'a> Iterator for DisplayModeIter<'a> {
    type Item = DisplayMode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|inner| DisplayMode { inner })
    }
}
