/// Load a specific global function
macro_rules! load_global_function {
    ($get_instance_proc_addr: expr, $name: expr) => {{
        let name = $name;
        unsafe { ($get_instance_proc_addr)(vulkan::VkInstance::null(), name.as_ptr()) }
            .map(|function| unsafe {
                $crate::FunctionSymbol::new(
                    std::mem::transmute(function),
                    $get_instance_proc_addr.shared_object().clone(),
                )
            })
            .ok_or_else(|| {
                $crate::Error::new(format!(
                    "unable to find \"{}\" Vulkan global function",
                    name.to_string_lossy()
                ))
            })
    }};
}
pub(in crate::gpu) use load_global_function;
