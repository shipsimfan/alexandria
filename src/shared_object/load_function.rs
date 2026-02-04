/// Load a function from a shared object
#[macro_export]
macro_rules! load_function {
    ($shared_object: expr, $name: expr) => {{
        let name = $name;
        let symbol = $shared_object.load_symbol::<std::ffi::c_void>(&name);
        let (symbol, shared_object) = unsafe { symbol.unwrap() };
        if symbol != std::ptr::null_mut() {
            Ok(unsafe { $crate::FunctionSymbol::new(std::mem::transmute(symbol), shared_object) })
        } else {
            Err($crate::Error::new(format!(
                "unable to load \"{}\" from \"{}\"",
                name.to_string_lossy(),
                shared_object.name().display()
            )))
        }
    }};
}
