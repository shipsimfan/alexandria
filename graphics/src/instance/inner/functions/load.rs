#[cfg(target_os = "windows")]
use crate::instance::Win32WindowSurfaceFunctions;
use crate::{
    GraphicsInstanceExtension, Result,
    instance::{
        GraphicsAdapterFunctions, GraphicsDebugMessengerFunctions, WindowSurfaceFunctions,
        inner::GraphicsInstanceFunctions,
    },
    util::load_instance_function,
};
use vulkan::{
    VK_DESTROY_INSTANCE, VK_ENUMERATE_PHYSICAL_DEVICES, VK_GET_DEVICE_PROC_ADDR, VkInstance,
};

impl GraphicsInstanceFunctions {
    /// Load all the required instance functions
    pub(in crate::instance) fn load(
        instance: VkInstance,
        extensions: &[GraphicsInstanceExtension],
    ) -> Result<GraphicsInstanceFunctions> {
        let mut debug_messenger = None;
        let mut surface = None;
        #[cfg(target_os = "windows")]
        let mut win32_surface = None;
        #[cfg(target_os = "linux")]
        let mut wayland_surface = None;

        for extension in extensions {
            match *extension {
                GraphicsInstanceExtension::DebugUtils => {
                    debug_messenger = Some(GraphicsDebugMessengerFunctions::load(instance)?)
                }
                GraphicsInstanceExtension::Surface => {
                    surface = Some(WindowSurfaceFunctions::load(instance)?)
                }
                #[cfg(target_os = "windows")]
                GraphicsInstanceExtension::Win32Surface => {
                    win32_surface = Some(Win32WindowSurfaceFunctions::load(instance)?)
                }
                #[cfg(target_os = "linux")]
                GraphicsInstanceExtension::WaylandSurface => {
                    use crate::instance::WaylandWindowSurfaceFunctions;

                    wayland_surface = Some(WaylandWindowSurfaceFunctions::load(instance)?)
                }
            }
        }

        Ok(GraphicsInstanceFunctions {
            adapter: GraphicsAdapterFunctions::load(instance)?,
            debug_messenger,
            surface,
            #[cfg(target_os = "windows")]
            win32_surface,
            #[cfg(target_os = "linux")]
            wayland_surface,

            get_device_proc_addr: load_instance_function!(instance, VK_GET_DEVICE_PROC_ADDR)?,
            enumerate_physical_devices: load_instance_function!(
                instance,
                VK_ENUMERATE_PHYSICAL_DEVICES
            )?,
            destroy_instance: load_instance_function!(instance, VK_DESTROY_INSTANCE)?,
        })
    }
}
