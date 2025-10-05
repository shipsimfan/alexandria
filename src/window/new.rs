use crate::{
    window::{WindowClass, WindowHandle},
    Adapter, DisplayMode, Result, Window,
};

impl Window {
    /// Create a new [`Window`] for rendering
    pub(in crate::window) fn new(
        title: &[u16],
        x: Option<i32>,
        y: Option<i32>,
        width: Option<u32>,
        height: Option<u32>,
        display_mode: DisplayMode,
        adapter: &Adapter,
    ) -> Result<Box<Self>> {
        assert!(title.last().is_some());
        assert_eq!(*title.last().unwrap(), 0);

        let class = WindowClass::register(&title)?;

        let mut window = Box::new_uninit();
        let handle = WindowHandle::create(
            &title,
            &class,
            x,
            y,
            width,
            height,
            display_mode,
            window.as_mut_ptr(),
        )?;

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
