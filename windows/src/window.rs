use crate::{Instance, Result};
use std::{ffi::c_int, mem::MaybeUninit, sync::Arc};

pub struct Window {
    h_wnd: win32::HWnd,
}

pub(super) extern "system" fn wnd_proc(
    h_wnd: win32::HWnd,
    msg: win32::UInt,
    w_param: win32::WParam,
    l_param: win32::LParam,
) -> win32::LResult {
    if msg == win32::WM_CREATE {
        let ptr: *mut Window = win32::CreateStruct::from_l_param(l_param).create_params() as _;
        win32::set_window_long_ptr(h_wnd, win32::GWLP_USERDATA, ptr as _).ok();
        win32::def_window_proc(h_wnd, msg, w_param, l_param)
    } else {
        let window: &mut Window = match win32::get_window_long_ptr(h_wnd, win32::GWLP_USERDATA) {
            Ok(window) => unsafe { &mut *(window as *mut _) },
            Err(_) => return win32::def_window_proc(h_wnd, msg, w_param, l_param),
        };
        window.wnd_proc(h_wnd, msg, w_param, l_param)
    }
}

impl Window {
    pub fn new(
        instance: &Instance,
        window_name: &str,
        width: usize,
        height: usize,
    ) -> Result<Box<Self>> {
        let mut window: Box<MaybeUninit<Self>> = Box::new_uninit();

        let h_wnd = win32::create_window_ex(
            &[],
            Some(win32::ClassName::Atom(instance.window_class())),
            Some(&win32::string_to_utf16!(window_name)),
            &[
                win32::WindowStyle::Caption,
                win32::WindowStyle::SysMenu,
                win32::WindowStyle::MinimizeBox,
                win32::WindowStyle::Visible,
            ],
            win32::CW_USEDEFAULT,
            win32::CW_USEDEFAULT,
            width as c_int,
            height as c_int,
            None,
            None,
            Some(win32::get_module_handle(None)?),
            Some(window.as_ptr() as _),
        )?;

        unsafe {
            window.as_mut_ptr().write(Window { h_wnd });

            Ok(window.assume_init())
        }
    }

    // Returns whether or not the window is still alive
    pub fn poll_events(&self) -> Result<bool> {
        let mut msg = win32::Msg::default();
        let mut result = true;

        while win32::peek_message(&mut msg, None, 0, 0, true) {
            if msg.message() == win32::WM_QUIT {
                result = false;
            }

            win32::translate_message(&msg);
            win32::dispatch_message(&msg);
        }

        Ok(result)
    }

    pub fn create_surface<L: vulkan::Loader>(
        &self,
        vk_instance: &Arc<vulkan::VkInstance<L>>,
    ) -> vulkan::Result<vulkan::VkSurfaceKHR<L>> {
        vk_instance.create_win32_surface(&vulkan::VkWin32SurfaceCreateInfoKHR::new(
            win32::get_module_handle(None).unwrap(),
            self.h_wnd,
        ))
    }

    pub(self) fn wnd_proc(
        &mut self,
        h_wnd: win32::HWnd,
        msg: win32::UInt,
        w_param: win32::WParam,
        l_param: win32::LParam,
    ) -> win32::LResult {
        match msg {
            win32::WM_CLOSE => {
                win32::post_quit_message(0);
                0
            }
            win32::WM_DESTROY => {
                win32::post_quit_message(0);
                0
            }
            _ => win32::def_window_proc(h_wnd, msg, w_param, l_param),
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        win32::destroy_window(self.h_wnd).ok();
    }
}

impl !Send for Window {}
