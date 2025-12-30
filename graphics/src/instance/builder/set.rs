use crate::{GraphicsInstanceBuilder, GraphicsVersion};
use std::borrow::Cow;

impl<'a> GraphicsInstanceBuilder<'a> {
    /// Set the version of Vulkan to use
    pub fn api_version(
        &mut self,
        api_version: GraphicsVersion,
    ) -> &mut GraphicsInstanceBuilder<'a> {
        self.api_version = api_version;
        self
    }

    /// Set the name and version of the application being run
    pub fn application<S: Into<Cow<'a, str>>>(
        &mut self,
        name: S,
        version: GraphicsVersion,
    ) -> &mut GraphicsInstanceBuilder<'a> {
        self.application = Some((name.into(), version));
        self
    }

    /// Set the name and version of the engine being run
    pub fn engine<S: Into<Cow<'a, str>>>(
        &mut self,
        name: S,
        version: GraphicsVersion,
    ) -> &mut GraphicsInstanceBuilder<'a> {
        self.engine = Some((name.into(), version));
        self
    }

    /// Add a new layer to the list of requested layers
    pub fn layer<S: Into<Cow<'a, str>>>(&mut self, layer: S) -> &mut GraphicsInstanceBuilder<'a> {
        self.layers.push(layer.into());
        self
    }

    /// Add new layers to the list of requested layers
    pub fn layers<S: Into<Cow<'a, str>>, I: IntoIterator<Item = S>>(
        &mut self,
        layers: I,
    ) -> &mut GraphicsInstanceBuilder<'a> {
        self.layers.extend(layers.into_iter().map(Into::into));
        self
    }
}
