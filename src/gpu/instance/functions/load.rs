use crate::{
    Result,
    gpu::{
        GpuSubsystem, VulkanInstanceExtension, VulkanInstanceFunctions,
        instance::{VulkanAdapterFunctions, VulkanDebugMessengerFunctions},
        load_instance_function,
    },
};
use vulkan::{
    VK_CREATE_DEVICE, VK_DESTROY_INSTANCE, VK_ENUMERATE_PHYSICAL_DEVICES, VK_GET_DEVICE_PROC_ADDR,
    VkInstance,
};

impl VulkanInstanceFunctions {
    /// Load all the required instance functions
    pub(in crate::gpu::instance) fn load(
        context: &GpuSubsystem,
        instance: VkInstance,
        extensions: &[VulkanInstanceExtension],
    ) -> Result<VulkanInstanceFunctions> {
        let mut debug_messenger = None;
        /*
        let mut surface = None;
        #[cfg(target_os = "windows")]
        let mut win32_surface = None;
        #[cfg(target_os = "linux")]
        let mut wayland_surface = None;
        */

        for extension in extensions {
            match *extension {
                VulkanInstanceExtension::DebugUtils => {
                    debug_messenger = Some(VulkanDebugMessengerFunctions::load(context, instance)?)
                }
                VulkanInstanceExtension::Surface => {
                    //surface = Some(WindowSurfaceFunctions::load(context, instance)?)
                }
                #[cfg(target_os = "windows")]
                VulkanInstanceExtension::Win32Surface => {
                    //win32_surface = Some(Win32WindowSurfaceFunctions::load(context, instance)?)
                }
                #[cfg(target_os = "linux")]
                VulkanInstanceExtension::WaylandSurface => {
                    //wayland_surface = Some(WaylandWindowSurfaceFunctions::load(context, instance)?)
                }
            }
        }

        Ok(VulkanInstanceFunctions {
            adapter: VulkanAdapterFunctions::load(context, instance)?,
            debug_messenger,

            enumerate_physical_devices: load_instance_function!(
                context,
                instance,
                VK_ENUMERATE_PHYSICAL_DEVICES
            )?,
            destroy_instance: load_instance_function!(context, instance, VK_DESTROY_INSTANCE)?,
            create_device: load_instance_function!(context, instance, VK_CREATE_DEVICE)?,
            get_device_proc_addr: load_instance_function!(
                context,
                instance,
                VK_GET_DEVICE_PROC_ADDR
            )?,
        })
    }
}
