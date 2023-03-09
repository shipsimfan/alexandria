use crate::{Adapter, Display, Instance, RefreshRate, Resolution, Result};
use win32::string_to_utf16;

pub struct Window {
    wnd: Option<win32::HWnd>,
}

mod create;

impl Window {
    pub fn new(
        title: &str,
        resolution: Option<Resolution>,
        refresh_rate: Option<RefreshRate>,
        instance: &mut Instance,
        adapter: Option<Adapter>,
        display: Option<Display>,
    ) -> Result<Self> {
        let mut adapter = match adapter {
            Some(adapter) => adapter,
            None => instance.default_adapter()?,
        };

        let display = match display {
            Some(display) => display,
            None => adapter.default_display()?,
        };

        let title = string_to_utf16!(title);

        let (resolution, _) = display.find_closest_resolution(resolution, refresh_rate);
        let (x, y) = create::calculate_window_position(resolution, &display);

        let wnd = create::create_window(
            instance,
            resolution.width(),
            resolution.height(),
            x,
            y,
            &title,
        )?;

        Ok(Window { wnd: Some(wnd) })
    }

    pub fn is_alive(&self) -> bool {
        self.wnd.is_some()
    }

    pub fn poll_events(&mut self) {
        while let Some(msg) = win32::peek_message(None, 0, 0, &[win32::PeekMessage::Remove]) {
            if msg.message() == win32::WindowMessage::Quit as u32 {
                self.wnd = None;
            }

            win32::translate_message(&msg);
            win32::dispatch_message(&msg);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.wnd.map(|wnd| win32::destroy_window(wnd).unwrap());
    }
}
