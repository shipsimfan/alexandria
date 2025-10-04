use crate::{
    window::{WindowClass, WindowHandle},
    Adapter, Result, Window,
};

impl Window {
    /// Create a new [`Window`] for rendering
    pub fn new(
        title: &str,
        width: u32,
        height: u32,
        adapter: Option<&Adapter>,
    ) -> Result<Box<Self>> {
        let mut utf16_title: Vec<_> = title.encode_utf16().collect();
        utf16_title.push(0);

        let class = WindowClass::register(&utf16_title)?;

        let mut window = Box::new_uninit();
        let handle = WindowHandle::create(&utf16_title, &class, width, height, window.as_mut_ptr())
            .map_err(|error| {
                eprintln!("Error: {}", error);
                error
            })?;

        Ok(Box::write(
            window,
            Window {
                is_running: true,
                handle,
                class,
            },
        ))
    }
}
