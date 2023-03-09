pub(super) extern "C" fn wnd_proc(
    wnd: win32::HWnd,
    msg: u32,
    w_param: win32::WParam,
    l_param: win32::LParam,
) -> isize {
    match msg {
        x if x == win32::WindowMessage::Close as u32 => {
            win32::destroy_window(wnd).ok();
            0
        }
        x if x == win32::WindowMessage::Destroy as u32 => {
            win32::post_quit_message(0);
            0
        }
        _ => win32::def_window_proc(wnd, msg, w_param, l_param),
    }
}
