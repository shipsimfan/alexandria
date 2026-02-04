use crate::gpu::{VulkanInstanceBuilder, VulkanInstanceExtension, VulkanVersion};
use std::borrow::Cow;

impl<'a> VulkanInstanceBuilder<'a> {
    /// Set the version of Vulkan to use
    pub fn api_version(&mut self, api_version: VulkanVersion) -> &mut VulkanInstanceBuilder<'a> {
        self.api_version = api_version;
        self
    }

    /// Set the name and version of the application being run
    pub fn application<S: Into<Cow<'a, str>>>(
        &mut self,
        name: S,
        version: VulkanVersion,
    ) -> &mut VulkanInstanceBuilder<'a> {
        self.application = Some((name.into(), version));
        self
    }

    /// Set the name and version of the engine being run
    pub fn engine<S: Into<Cow<'a, str>>>(
        &mut self,
        name: S,
        version: VulkanVersion,
    ) -> &mut VulkanInstanceBuilder<'a> {
        self.engine = Some((name.into(), version));
        self
    }

    /// Add a new layer to the list of requested layers
    pub fn layer<S: Into<Cow<'a, str>>>(&mut self, layer: S) -> &mut VulkanInstanceBuilder<'a> {
        self.layers.push(layer.into());
        self
    }

    /// Add new layers to the list of requested layers
    pub fn layers<S: Into<Cow<'a, str>>, I: IntoIterator<Item = S>>(
        &mut self,
        layers: I,
    ) -> &mut VulkanInstanceBuilder<'a> {
        self.layers.extend(layers.into_iter().map(|s| s.into()));
        self
    }

    /// Add a new extension to the list of requested extensions
    pub fn extension(
        &mut self,
        extension: VulkanInstanceExtension,
    ) -> &mut VulkanInstanceBuilder<'a> {
        self.extensions.push(extension);
        self
    }

    /// Add new extensions to the list of requested extensions
    pub fn extensions<I: IntoIterator<Item = VulkanInstanceExtension>>(
        &mut self,
        extensions: I,
    ) -> &mut VulkanInstanceBuilder<'a> {
        self.extensions.extend(extensions);
        self
    }

    /*
    /// Add the required extensions for create surfaces for `window`
    #[cfg(target_os = "windows")]
    #[allow(unused_variables)]
    pub fn window_extensions<Callbacks: WindowEvents>(
        &mut self,
        window: &Window<Callbacks>,
    ) -> &mut VulkanInstanceBuilder<'a> {
        self.extensions([
            VulkanInstanceExtension::Surface,
            VulkanInstanceExtension::Win32Surface,
        ])
    }

    /// Add the required extensions for create surfaces for `window`
    #[cfg(target_os = "linux")]
    #[allow(unused_variables)]
    pub fn window_extensions<Callbacks: WindowEvents>(
        &mut self,
        window: &Window<Callbacks>,
    ) -> &mut VulkanInstanceBuilder<'a> {
        self.extensions([
            VulkanInstanceExtension::Surface,
            match window {
                Window::Wayland(_) => VulkanInstanceExtension::WaylandSurface,
                Window::X11(_) => todo!(),
            },
        ])
    }
    */
}
