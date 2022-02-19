use crate::{
    graphics::{Graphics, GraphicsCreationError},
    input::Input,
    RenderError,
};
use std::{ffi::CString, ptr::null};

pub struct Window {
    h_wnd: win32::HWnd,
    msg: win32::Msg,
    input: Input,
    graphics: Option<Graphics>,
}

#[derive(Debug)]
pub enum WindowCreationError {
    WindowCreationError(win32::Win32Error),
    GraphicsCreationError(GraphicsCreationError),
}

extern "C" fn message_router(
    h_wnd: win32::HWnd,
    msg: win32::UInt,
    w_param: win32::WParam,
    l_param: win32::LParam,
) -> win32::LResult {
    let app: &mut Window = if msg == win32::WM_CREATE {
        let ptr = win32::CreateStructA::from_l_param(l_param).create_params() as *mut _;
        win32::set_window_long_ptr(h_wnd, win32::GWLP_USERDATA, ptr as *const _);
        unsafe { &mut *ptr }
    } else {
        unsafe { &mut *(win32::get_window_long_ptr(h_wnd, win32::GWLP_USERDATA) as *mut _) }
    };

    app.wnd_proc(h_wnd, msg, w_param, l_param)
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Result<Box<Self>, WindowCreationError> {
        // Create window box
        let mut window = Box::new(Window {
            h_wnd: null(),
            msg: win32::Msg::default(),
            input: Input::new(),
            graphics: None,
        });

        // Register window class
        let window_name = CString::new(title).unwrap();
        let wnd_class = win32::WndClassEx::new(
            &[win32::Cs::OwnDC, win32::Cs::HRedraw, win32::Cs::VRedraw],
            message_router,
            0,
            0,
            None,
            None,
            None,
            None,
            None,
            &window_name,
            None,
        );
        win32::register_class_ex(&wnd_class)?;

        // Create window
        window.h_wnd = win32::create_window_ex(
            &[],
            &window_name,
            &window_name,
            &[
                win32::Ws::Border,
                win32::Ws::Caption,
                win32::Ws::MinimizeBox,
                win32::Ws::SysMenu,
                win32::Ws::Visible,
            ],
            None,
            None,
            width as i32,
            height as i32,
            None,
            None,
            None,
            Some(window.as_ref() as *const _ as *const _),
        )?;

        window.graphics = Some(Graphics::new(window.h_wnd, width as u32, height as u32)?);

        Ok(window)
    }

    pub fn poll_message(&mut self) -> bool {
        self.input.frame_reset();

        while win32::peek_message(&mut self.msg, None, 0, 0, &[win32::Pm::Remove]) {
            if self.msg.message == win32::WM_QUIT {
                return false;
            }

            win32::translate_message(&self.msg);
            win32::dispatch_message(&self.msg);
        }

        true
    }

    pub fn device_context(&mut self) -> &mut win32::ID3D11DeviceContext {
        self.graphics.as_mut().unwrap().device_context()
    }

    pub fn device(&mut self) -> &mut win32::ID3D11Device {
        self.graphics.as_mut().unwrap().device()
    }

    pub fn begin_render(&mut self, clear_color: [f32; 4]) {
        self.graphics.as_mut().unwrap().begin_render(clear_color);
    }

    pub fn end_render(&mut self) -> Result<(), RenderError> {
        self.graphics.as_mut().unwrap().end_render()
    }

    pub fn get_key(&self, key: u8) -> bool {
        self.input.get_key(key)
    }

    pub fn get_key_down(&self, key: u8) -> bool {
        self.input.get_key_down(key)
    }

    pub fn get_key_up(&self, key: u8) -> bool {
        self.input.get_key_up(key)
    }

    fn wnd_proc(
        &mut self,
        h_wnd: win32::HWnd,
        msg: win32::UInt,
        w_param: win32::WParam,
        l_param: win32::LParam,
    ) -> win32::LResult {
        match msg {
            win32::WM_DESTROY => win32::post_quit_message(0),
            win32::WM_CLOSE => win32::destroy_window(h_wnd).unwrap_or(()),
            win32::WM_KEYDOWN => self.input.key_down(w_param as u8),
            win32::WM_KEYUP => self.input.key_up(w_param as u8),
            _ => return win32::def_window_proc(h_wnd, msg, w_param, l_param),
        }

        0
    }
}

impl std::error::Error for WindowCreationError {}

impl std::fmt::Display for WindowCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WindowCreationError::WindowCreationError(error) =>
                    format!("Unable to create window ({})", error),
                WindowCreationError::GraphicsCreationError(error) => format!("{}", error),
            }
        )
    }
}

impl From<GraphicsCreationError> for WindowCreationError {
    fn from(error: GraphicsCreationError) -> Self {
        WindowCreationError::GraphicsCreationError(error)
    }
}

impl From<win32::Win32Error> for WindowCreationError {
    fn from(error: win32::Win32Error) -> Self {
        WindowCreationError::WindowCreationError(error)
    }
}
