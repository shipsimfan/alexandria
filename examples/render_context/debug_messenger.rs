#[cfg(debug_assertions)]
mod debug_messenger {
    use alexandria::gpu::{
        GpuSubsystem, VulkanDebugMessageSeverityFlag, VulkanDebugMessageTypeFlag,
        VulkanDebugMessageTypeFlags, VulkanDebugMessenger, VulkanDebugMessengerCallback,
        VulkanInstance, VulkanInstanceExtension,
    };

    pub struct DebugCallback;

    pub type DebugMessenger = VulkanDebugMessenger<DebugCallback>;

    impl VulkanDebugMessengerCallback for DebugCallback {
        fn message(
            &self,
            message: &str,
            severity: VulkanDebugMessageSeverityFlag,
            _: VulkanDebugMessageTypeFlags,
        ) {
            println!("[{:?}] {}", severity, message);
        }
    }

    /// Does this system have required Vulkan validation layers?
    pub fn has_layers(gpu: &GpuSubsystem) -> Option<&'static [&'static str]> {
        for layer in gpu.layers().expect("unable to get layers") {
            if layer.name() == "VK_LAYER_KHRONOS_validation" {
                return Some(&["VK_LAYER_KHRONOS_validation"]);
            }
        }

        None
    }

    /// Does this system have required Vulkan validation extensions?
    pub fn has_extensions(gpu: &GpuSubsystem) -> Option<&'static [VulkanInstanceExtension]> {
        for extension in gpu.extensions(None).expect("unable to get extensions") {
            if extension == VulkanInstanceExtension::DebugUtils {
                return Some(&[VulkanInstanceExtension::DebugUtils]);
            }
        }

        None
    }

    /// Create a new debug messenger
    pub fn create(instance: &VulkanInstance) -> VulkanDebugMessenger<DebugCallback> {
        instance
            .create_debug_messenger(
                VulkanDebugMessageSeverityFlag::ErrorExt
                    | VulkanDebugMessageSeverityFlag::WarningExt
                    | VulkanDebugMessageSeverityFlag::InfoExt
                    | VulkanDebugMessageSeverityFlag::VerboseExt,
                VulkanDebugMessageTypeFlag::AddressBindingExt
                    | VulkanDebugMessageTypeFlag::GeneralExt
                    | VulkanDebugMessageTypeFlag::PerformanceExt
                    | VulkanDebugMessageTypeFlag::ValidationExt,
                DebugCallback,
            )
            .expect("unable to create debug messenger")
    }
}

#[cfg(not(debug_assertions))]
mod debug_messenger {
    pub type DebugMessenger = ();

    /// Does this system have required Vulkan validation layers?
    pub fn has_layers(_: &GpuSubsystem) -> Option<&'static [&'static str]> {
        Some(&[])
    }

    /// Does this system have required Vulkan validation extensions?
    pub fn has_extensions(_: &GpuSubsystem) -> Option<&'static [VulkanInstanceExtension]> {
        Some(&[])
    }

    /// Create a new debug messenger
    pub fn create(_: &VulkanInstance) -> () {
        ()
    }
}

pub use debug_messenger::*;
