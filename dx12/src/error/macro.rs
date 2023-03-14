#[macro_export]
macro_rules! raw_error {
    ($kind:ident, $error:expr) => {
        crate::Error::new(crate::ErrorKind::$kind, $error)
    };
}

#[macro_export]
macro_rules! instance_error {
    ($kind:ident, $error:expr, $instance_debug:expr) => {{
        let mut error = crate::Error::new(crate::ErrorKind::$kind, $error);
        $instance_debug.as_ref().map(|instance| {
            let mut instance = instance.lock().unwrap();
            error.get_instance_debug_messages(&mut instance)
        });
        error
    }};
}

#[macro_export]
macro_rules! graphics_3d_error {
    ($kind:ident, $error:expr, $graphics_3d_debug:expr) => {{
        let mut error = crate::Error::new(crate::ErrorKind::$kind, $error);
        $graphics_3d_debug.as_ref().map(|graphics_3d_debug| {
            let mut graphics_3d_debug = graphics_3d_debug.lock().unwrap();
            error.get_3d_debug_messages(&mut graphics_3d_debug)
        });
        error
    }};
}

#[macro_export]
macro_rules! map_raw_error {
    ($expr:expr, $kind:ident) => {
        $expr.map_err(|error| crate::raw_error!($kind, error))
    };
}

#[macro_export]
macro_rules! map_instance_error {
    ($expr:expr, $kind:ident, $instance_debug:expr) => {{
        $expr.map_err(|error| crate::instance_error!($kind, error, $instance_debug))
    }};
}

#[macro_export]
macro_rules! map_3d_error {
    ($expr:expr, $kind:ident, $graphics_3d_debug:expr) => {{
        $expr.map_err(|error| crate::graphics_3d_error!($kind, error, $graphics_3d_debug))
    }};
}
