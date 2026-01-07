use crate::{GraphicsInstance, instance::GraphicsInstanceInner};
use std::ops::Deref;

impl Deref for GraphicsInstance {
    type Target = GraphicsInstanceInner;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}
