use crate::{
    graphics::{Adapter, RenderContext},
    window::{WindowClass, WindowHandle},
    DisplayMode, Result, Window,
};

impl Window {
    /// Create a new [`Window`] for rendering
    pub(in crate::window) fn new(
        title: &[u16],
        x: Option<i32>,
        y: Option<i32>,
        width: Option<u32>,
        height: Option<u32>,
        vsync: bool,
        display_mode: DisplayMode,
        adapter: &mut Adapter,
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

        let (position, size) = handle.get_size_and_position()?;

        let (render_context, graphics_context) =
            RenderContext::new(&handle, adapter, size.x, size.y)?;

        Ok(Box::write(
            window,
            Window {
                is_running: true,
                position,
                size,
                vsync,
                display_mode,
                render_context,
                graphics_context,
                in_move: false,
                wnd_proc_result: Ok(()),
                handle,
                class,
            },
        ))
    }
}
