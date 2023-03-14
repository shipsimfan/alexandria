use crate::{Adapter, Display, Instance, RefreshRate, Resolution, Result};
use win32::string_to_utf16;

pub struct Window {
    wnd: win32::HWnd,
    alive: bool,

    graphics_3d: Graphics3D,
}

mod create;
mod wnd_proc;

pub(crate) mod graphics_3d;

pub(crate) use wnd_proc::wnd_proc;

pub use graphics_3d::Graphics3D;

impl Window {
    pub fn new(
        title: &str,
        resolution: Option<Resolution>,
        refresh_rate: Option<RefreshRate>,
        instance: &mut Instance,
        adapter: Option<Adapter>,
        display: Option<Display>,
    ) -> Result<Self> {
        // Unwrap the adapter and display
        let mut adapter = match adapter {
            Some(adapter) => adapter,
            None => instance.default_adapter()?,
        };

        let display = match display {
            Some(display) => display,
            None => adapter.default_display()?,
        };

        // Convert the provided values
        let title = string_to_utf16!(title);
        let (resolution, _) = display.find_closest_resolution(resolution, refresh_rate);
        let (x, y) = create::calculate_window_position(resolution, &display);

        // Create the window
        let wnd = create::create_window(
            instance,
            resolution.width(),
            resolution.height(),
            x,
            y,
            &title,
        )?;

        // Create the 3D graphics context
        let graphics_3d = Graphics3D::new(instance, &mut adapter, resolution, wnd)?;

        Ok(Window {
            wnd,
            alive: true,
            graphics_3d,
        })
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn graphics_3d(&self) -> &Graphics3D {
        &self.graphics_3d
    }

    pub fn poll_events(&mut self) {
        while let Some(msg) = win32::peek_message(None, 0, 0, &[win32::PeekMessage::Remove]) {
            if msg.message() == win32::WindowMessage::Quit as u32 {
                self.alive = false;
            }

            win32::translate_message(&msg);
            win32::dispatch_message(&msg);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        win32::destroy_window(self.wnd).unwrap();
    }
}

impl !Send for Window {}
