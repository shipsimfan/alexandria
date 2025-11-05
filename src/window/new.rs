use crate::{
    graphics::{Adapter, RenderContext},
    input::{InputDevice, InputDeviceKind, InputDeviceMetadata},
    window::{WindowClass, WindowHandle},
    DisplayMode, Result, Window,
};

impl<LogCallbacks: crate::LogCallbacks, Input: crate::input::Input> Window<LogCallbacks, Input> {
    /// Create a new [`Window`] for rendering
    pub(in crate::window) fn new(
        title: &[u16],
        x: Option<i32>,
        y: Option<i32>,
        width: Option<u32>,
        height: Option<u32>,
        vsync: bool,
        display_mode: DisplayMode,
        mut input: Input,
        mut log_callbacks: LogCallbacks,
        adapter: &mut Adapter,
    ) -> Result<Box<Self>> {
        assert!(title.last().is_some());
        assert_eq!(*title.last().unwrap(), 0);

        // Create window class
        let class = WindowClass::register::<LogCallbacks, Input>(&title)?;

        // Create window
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

        // Get position and size
        let (position, size) = handle.get_size_and_position()?;

        // Create render context
        let (render_context, graphics_context) =
            RenderContext::new(&handle, adapter, size.x, size.y, &mut log_callbacks)?;

        // Register basic input devices
        let keyboard_id = input.device_connected(InputDevice::new(
            InputDeviceKind::Keyboard,
            "keyboard".into(),
            InputDeviceMetadata::new(0, 0, 0, 1, 6),
            u8::MAX,
            0,
        ));

        // Write info into output box
        Ok(Box::write(
            window,
            Window {
                is_running: true,
                position,
                size,
                vsync,
                display_mode,
                is_focused: true,
                in_move: false,
                input,
                keyboard_id,
                render_context,
                graphics_context,
                log_callbacks,
                wnd_proc_result: Ok(()),
                handle,
                class,
            },
        ))
    }
}
