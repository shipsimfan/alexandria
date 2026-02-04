use crate::AlexandriaContextInner;
use functions::GlobalVulkanFunctions;
use std::{mem::MaybeUninit, sync::Weak};

mod functions;

mod all_extensions;
mod extensions;
mod instance_builder;
mod layers;
mod new;
mod version;

/// Allows interaction and control over GPUs
pub struct GpuSubsystem {
    /// The context this subsystem is a part of
    context: Weak<MaybeUninit<AlexandriaContextInner>>,

    /// The Vulkan functions not specific to any Vulkan instance
    pub(in crate::gpu) functions: GlobalVulkanFunctions,
}
