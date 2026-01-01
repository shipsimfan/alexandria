/// Load a specific global function
macro_rules! load_global_function {
    ($name: expr) => {{
        let name = $name;
        let function =
            unsafe { vulkan::vkGetInstanceProcAddr(vulkan::VkInstance::null(), name.as_ptr()) }
                .ok_or_else(|| {
                    crate::GraphicsError::new(format!(
                        "unable to find \"{}\" global function",
                        name.display()
                    ))
                })?;

        Ok(unsafe { std::mem::transmute(function) })
    }};
}
pub(crate) use load_global_function;
