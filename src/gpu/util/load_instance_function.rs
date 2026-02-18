/// Load a specific instance function
macro_rules! load_instance_function {
    ($context: expr, $instance: expr, $name: expr) => {{
        let name = $name;
        unsafe { ($context.functions().get_instance_proc_addr)($instance, name.as_ptr()) }
            .map(|function| unsafe {
                $crate::FunctionSymbol::new(
                    std::mem::transmute(function),
                    $context
                        .functions()
                        .get_instance_proc_addr
                        .shared_object()
                        .clone(),
                )
            })
            .ok_or_else(|| {
                crate::Error::new(format!(
                    "unable to find \"{}\" Vulkan instance function",
                    name.to_string_lossy()
                ))
            })
    }};
}
pub(in crate::gpu) use load_instance_function;
