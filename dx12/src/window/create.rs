use crate::{Display, Instance, Resolution, Result};

pub(super) fn create_window(
    instance: &Instance,
    width: usize,
    height: usize,
    x: isize,
    y: isize,
    title: &[u16],
) -> Result<win32::HWnd> {
    win32::create_window_ex(
        &[],
        instance.window_class().as_class_name(),
        title,
        &[
            win32::WindowStyle::Caption,
            win32::WindowStyle::SysMenu,
            win32::WindowStyle::MinimizeBox,
            win32::WindowStyle::Visible,
        ],
        x as i32,
        y as i32,
        width as i32,
        height as i32,
        None,
        None,
        Some(instance.instance_handle()),
        None,
    )
}

pub(super) fn calculate_window_position(
    resolution: Resolution,
    display: &Display,
) -> (isize, isize) {
    let position = display.position();
    let size = display.size();

    (
        position.0 + (size.0 / 2) - (resolution.width() as isize / 2),
        position.1 + (size.1 / 2) - (resolution.height() as isize / 2),
    )
}
