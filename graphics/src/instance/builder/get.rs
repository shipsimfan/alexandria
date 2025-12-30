use crate::{GraphicsInstanceBuilder, GraphicsVersion};
use std::borrow::Cow;

impl<'a> GraphicsInstanceBuilder<'a> {
    /// Get the requested Vulkan version
    pub fn get_api_version(&self) -> GraphicsVersion {
        self.api_version
    }

    /// Get the name and version of the application being run
    pub fn get_application(&self) -> Option<(&str, GraphicsVersion)> {
        self.application
            .as_ref()
            .map(|(str, version)| (str.as_ref(), *version))
    }

    /// Get the name of the application being run
    pub fn get_application_name(&self) -> Option<&str> {
        self.get_application().map(|(name, _)| name)
    }

    /// Get the name of the application being run
    pub fn get_application_version(&self) -> Option<GraphicsVersion> {
        self.get_application().map(|(_, version)| version)
    }

    /// Get the name and version of the engine being run
    pub fn get_engine(&self) -> Option<(&str, GraphicsVersion)> {
        self.engine
            .as_ref()
            .map(|(str, version)| (str.as_ref(), *version))
    }

    /// Get the name of the engine being run
    pub fn get_engine_name(&self) -> Option<&str> {
        self.get_engine().map(|(name, _)| name)
    }

    /// Get the name of the engine being run
    pub fn get_engine_version(&self) -> Option<GraphicsVersion> {
        self.get_engine().map(|(_, version)| version)
    }

    /// Get requested layers for the instance
    pub fn get_layers(&self) -> &[Cow<'a, str>] {
        &self.layers
    }
}
