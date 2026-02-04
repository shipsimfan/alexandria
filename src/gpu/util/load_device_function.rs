/// Load a specific device function
macro_rules! load_device_function {
    ($instance: expr, $device: expr, $name: expr) => {{
        let name = $name;
        unsafe { ($instance.functions.get_device_proc_addr)($device, name.as_ptr()) }
            .map(|function| unsafe {
                $crate::FunctionSymbol::new(
                    std::mem::transmute(function),
                    $instance
                        .functions
                        .get_device_proc_addr
                        .shared_object()
                        .clone(),
                )
            })
            .ok_or_else(|| {
                crate::Error::new(format!(
                    "unable to find \"{}\" Vulkan device function",
                    name.to_string_lossy()
                ))
            })
    }};
}
pub(in crate::gpu) use load_device_function;
