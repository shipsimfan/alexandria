use crate::{
    math::Vector2,
    window::display::linux::wayland::{WaylandDisplayEventHandler, XdgOutputListener},
};
use std::ffi::CStr;

impl<UserEvent: 'static + Send> XdgOutputListener for WaylandDisplayEventHandler<UserEvent> {
    fn logical_position(&mut self, x: i32, y: i32) {
        let new_position = Vector2::new(x, y);
        if self.rect.position != new_position {
            self.rect.position = new_position;
            self.moved = true;

            self.work_area.position = self.rect.position;
            self.work_area_changed = true;
        }
    }

    fn logical_size(&mut self, width: i32, height: i32) {
        let new_content_scale = if width > 0 && height > 0 {
            self.logical_size = Vector2::new(width as _, height as _);
            self.work_area.size.x as f32 / self.logical_size.x as f32
        } else {
            1.0
        };

        if self.content_scale != new_content_scale {
            self.content_scale = new_content_scale;
            self.content_scale_changed = true;
        }
    }

    fn name(&mut self, name: &CStr) {
        if self.name.len() == 0 {
            self.name = name.to_string_lossy().into_owned();
        }

        if self.id.len() == 0 {
            self.id = name.to_string_lossy().into_owned();
        }
    }

    fn description(&mut self, description: &CStr) {
        if self.name.len() == 0 {
            self.name = description.to_string_lossy().into_owned();
        }
    }
}
