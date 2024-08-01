use super::Instance;
use crate::functions::SurfaceFunctions;
use std::{ptr::null, sync::Arc};
use vulkan::VkSurfaceKHR;

/// A surface from which can be drawn on
pub struct Surface {
    /// The handle to the surface
    handle: VkSurfaceKHR,

    /// The instance this surface was created for
    instance: Arc<Instance>,
}

impl Surface {
    /// Creates a new [`Surface`]
    #[allow(unused_variables)]
    pub(crate) fn new(handle: VkSurfaceKHR, instance: Arc<Instance>) -> Self {
        Surface { handle, instance }
    }

    /// Gets the handle to the underlying Vulkan surface
    pub(crate) fn handle(&self) -> VkSurfaceKHR {
        self.handle
    }

    /// Gets the surface Vulkan functions
    pub(crate) fn f(&self) -> &SurfaceFunctions {
        self.instance.f().s().unwrap()
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.f()
            .destroy_surface(self.instance.handle(), self.handle, null());
    }
}
