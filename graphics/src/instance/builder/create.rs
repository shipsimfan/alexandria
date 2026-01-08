use crate::{GraphicsInstance, GraphicsInstanceBuilder, Result, instance::GraphicsInstanceInner};
use std::sync::Arc;

impl<'a> GraphicsInstanceBuilder<'a> {
    /// Create a new [`GraphicsInstance`] with the provided settings
    pub fn create(&self) -> Result<GraphicsInstance> {
        Ok(GraphicsInstance {
            inner: Arc::new(GraphicsInstanceInner::new(
                self.api_version,
                self.application.as_ref().map(|(s, v)| (s.as_ref(), *v)),
                self.engine.as_ref().map(|(s, v)| (s.as_ref(), *v)),
                &self.extensions,
                &self.layers,
            )?),
        })
    }
}
