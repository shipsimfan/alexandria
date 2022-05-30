use crate::{
    graphics::{Graphics, GraphicsCreationError},
    Input, RenderError, StateTrackingInput,
};
use std::{ffi::CString, ptr::null, rc::Rc};

pub struct Window<I: Input = StateTrackingInput> {
    h_wnd: win32::HWnd,
    msg: win32::Msg,
    input: I,
    graphics: Option<Graphics>,
    width: usize,
    height: usize,
    mouse_center: (i32, i32),
    update_mouse_center: bool,
}

#[derive(Debug)]
pub enum WindowCreationError {
    WindowCreationError(win32::Win32Error),
    GraphicsCreationError(GraphicsCreationError),
}

extern "C" fn message_router<I: Input>(
    h_wnd: win32::HWnd,
    msg: win32::UInt,
    w_param: win32::WParam,
    l_param: win32::LParam,
) -> win32::LResult {
    let app: &mut Window<I> = if msg == win32::WM_CREATE {
        let ptr = win32::CreateStructA::from_l_param(l_param).create_params() as *mut _;
        win32::set_window_long_ptr(h_wnd, win32::GWLP_USERDATA, ptr as *const _);
        unsafe { &mut *ptr }
    } else {
        unsafe { &mut *(win32::get_window_long_ptr(h_wnd, win32::GWLP_USERDATA) as *mut _) }
    };

    app.wnd_proc(h_wnd, msg, w_param, l_param)
}

const STYLE: [win32::Ws; 5] = [
    win32::Ws::Border,
    win32::Ws::Caption,
    win32::Ws::MinimizeBox,
    win32::Ws::SysMenu,
    win32::Ws::Visible,
];

impl<I: Input> Window<I> {
    pub fn new(title: &str, width: usize, height: usize) -> Result<Box<Self>, WindowCreationError> {
        // Create window box
        let mut window = Box::new(Window {
            h_wnd: null(),
            msg: win32::Msg::default(),
            input: I::new(),
            graphics: None,
            width,
            height,
            mouse_center: (0, 0),
            update_mouse_center: true,
        });

        // Register window class
        let window_name = CString::new(title).unwrap();
        let wnd_class = win32::WndClassEx::new(
            &[win32::Cs::OwnDC, win32::Cs::HRedraw, win32::Cs::VRedraw],
            message_router::<I>,
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
        let mut rect = win32::Rect::default();
        rect.right = width as i32;
        rect.bottom = height as i32;
        win32::adjust_window_rect(&mut rect, &STYLE, false)?;

        window.h_wnd = win32::create_window_ex(
            &[],
            &window_name,
            &window_name,
            &STYLE,
            None,
            None,
            rect.right - rect.left,
            rect.bottom - rect.top,
            None,
            None,
            None,
            Some(window.as_ref() as *const _ as *const _),
        )?;

        window.graphics = Some(Graphics::new(window.h_wnd, width as u32, height as u32)?);

        window.update_mouse_center();

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

    pub fn begin_render(&mut self, clear_color: [f32; 4]) {
        self.graphics.as_mut().unwrap().begin_render(clear_color);
    }

    pub fn end_render(&mut self) -> Result<(), RenderError> {
        self.graphics.as_mut().unwrap().end_render()
    }

    pub fn set_mouse_lock(&mut self, state: bool) {
        self.input.set_mouse_lock(state);

        let state = self.input.is_mouse_locked();

        win32::show_cursor(!state);
        if state {
            self.reset_mouse_position();
        }
    }

    pub fn input_mut(&mut self) -> &mut I {
        &mut self.input
    }

    pub fn input(&self) -> &I {
        &self.input
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn device(&self) -> &Rc<win32::ID3D11Device> {
        self.graphics.as_ref().unwrap().device()
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
            win32::WM_WINDOWPOSCHANGED => {
                self.update_mouse_center = true;
            }
            win32::WM_KEYDOWN => self.input.key_down(w_param as u8),
            win32::WM_KEYUP => self.input.key_up(w_param as u8),
            win32::WM_LBUTTONDOWN => self.input.mouse_down(0),
            win32::WM_LBUTTONUP => self.input.mouse_up(0),
            win32::WM_RBUTTONDOWN => self.input.mouse_down(1),
            win32::WM_RBUTTONUP => self.input.mouse_up(1),
            win32::WM_MBUTTONDOWN => self.input.mouse_down(2),
            win32::WM_MBUTTONUP => self.input.mouse_up(2),
            win32::WM_MOUSEMOVE => {
                let x = (l_param & 0xFFFF) as i16;
                let y = (l_param.wrapping_shr(16) & 0xFFFF) as i16;

                let width2 = self.width as isize / 2;
                let height2 = self.height as isize / 2;

                self.input
                    .update_mouse_position((x as isize - width2, y as isize - height2));

                if self.input.is_mouse_locked() {
                    self.reset_mouse_position();
                }
            }
            win32::WM_SETFOCUS => {
                if self.input.is_mouse_locked() {
                    self.reset_mouse_position();
                }
            }
            _ => return win32::def_window_proc(h_wnd, msg, w_param, l_param),
        }

        0
    }

    fn reset_mouse_position(&mut self) {
        if self.update_mouse_center {
            self.update_mouse_center()
        }

        win32::set_cursor_pos(self.mouse_center.0, self.mouse_center.1)
            .expect("Failed to set mouse position!");
    }

    fn update_mouse_center(&mut self) {
        self.mouse_center = win32::client_to_screen(
            self.h_wnd,
            (self.width / 2) as i32,
            (self.height / 2) as i32,
        )
        .expect("Failed to convert coordinates!");
        self.update_mouse_center = false;
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
