/// Load a specific device function
macro_rules! load_device_function {
    ($instance: expr, $device: expr, $name: expr) => {{
        let name = $name;
        let function = ($instance.functions.get_device_proc_addr)($device, name.as_ptr())
            .ok_or_else(|| {
                crate::GraphicsError::new(format!(
                    "unable to find \"{}\" device function",
                    name.display()
                ))
            })?;

        Ok(unsafe { std::mem::transmute(function) })
    }};
}
pub(crate) use load_device_function;
