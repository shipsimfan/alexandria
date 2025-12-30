/// Load a specific instance function
macro_rules! load_instance_function {
    ($instance: expr, $name: expr) => {{
        let name = $name;
        let function = unsafe { vulkan::vkGetInstanceProcAddr($instance, name.as_ptr()) }
            .ok_or_else(|| {
                crate::GraphicsError::new(format!(
                    "unable to find \"{}\" instance function",
                    name.display()
                ))
            })?;

        Ok(unsafe { std::mem::transmute(function) })
    }};
}
pub(crate) use load_instance_function;
