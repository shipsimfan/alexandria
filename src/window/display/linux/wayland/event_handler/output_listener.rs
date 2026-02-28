use crate::{
    EventKind,
    math::{Rational, Vector2i, Vector2u},
    window::{
        DisplayOrientation,
        display::linux::wayland::{WaylandDisplayEventHandler, WlOutputListener},
    },
};
use std::{ffi::CStr, num::NonZeroU32};
use wayland::{wl_output_mode, wl_output_transform};

impl<UserEvent: 'static + Send> WlOutputListener for WaylandDisplayEventHandler<UserEvent> {
    fn geometry(
        &mut self,
        x: i32,
        y: i32,
        physical_width: i32,
        physical_height: i32,
        make: &CStr,
        model: &CStr,
        transform: wl_output_transform,
    ) {
        if physical_width > 0 && physical_height > 0 {
            self.physical_size = Some(Vector2u::new(physical_width as _, physical_height as _));
        }

        if self.name.len() == 0 {
            self.name = format!("{} {}", make.to_string_lossy(), model.to_string_lossy());
        }

        let new_position = Vector2i::new(x, y);
        if self.rect.position != new_position {
            self.rect.position = new_position;
            self.moved = true;

            self.work_area.position = new_position;
            self.work_area_changed = true;
        }

        let new_orientation = match transform {
            wl_output_transform::Normal | wl_output_transform::Flipped => {
                DisplayOrientation::Landscape
            }
            wl_output_transform::Rotate90 | wl_output_transform::Flipped90 => {
                DisplayOrientation::Portrait
            }
            wl_output_transform::Rotate180 | wl_output_transform::Flipped180 => {
                DisplayOrientation::LandscapeFlipped
            }
            wl_output_transform::Rotate270 | wl_output_transform::Flipped270 => {
                DisplayOrientation::PortraitFlipped
            }
            _ => DisplayOrientation::Landscape,
        };

        if self.orientation != new_orientation {
            self.orientation = new_orientation;
            self.rotated = true;
        }
    }

    fn mode(&mut self, flags: i32, width: i32, height: i32, refresh: i32) {
        // Check flags for current and set the display's size and refresh rate if it's the current mode
        if flags & (wl_output_mode::Current as i32) == 0 {
            return;
        }

        let new_size = Vector2i::new(width, height);
        if self.rect.size != new_size {
            self.rect.size = new_size;
            self.resized = true;

            self.work_area.size = new_size;
            self.work_area_changed = true;
        }

        let new_refresh_rate = Rational::new(refresh, NonZeroU32::new(1000).unwrap());
        if self.refresh_rate != new_refresh_rate {
            self.refresh_rate = new_refresh_rate;
            self.refresh_rate_changed = true;
        }
    }

    fn done(&mut self) {
        if self.moved {
            self.moved = false;

            if self.events_enabled {
                self.event_queue
                    .push(EventKind::DisplayMoved {
                        id: self.display_id.unwrap(),
                        new_position: self.rect.position,
                    })
                    .unwrap(); // TODO: Add error handling
            }
        }

        if self.resized {
            self.resized = false;

            if self.events_enabled {
                self.event_queue
                    .push(EventKind::DisplayResized {
                        id: self.display_id.unwrap(),
                        new_size: self.rect.size,
                    })
                    .unwrap(); // TODO: Add error handling
            }
        }

        if self.work_area_changed {
            self.work_area_changed = false;

            if self.events_enabled {
                self.event_queue
                    .push(EventKind::DisplayWorkAreaChanged {
                        id: self.display_id.unwrap(),
                        new_work_area: self.work_area,
                    })
                    .unwrap(); // TODO: Add error handling
            }
        }

        if self.refresh_rate_changed {
            self.refresh_rate_changed = false;

            if self.events_enabled {
                self.event_queue
                    .push(EventKind::DisplayRefreshRateChanged {
                        id: self.display_id.unwrap(),
                        new_refresh_rate: self.refresh_rate,
                    })
                    .unwrap(); // TODO: Add error handling
            }
        }

        if self.rotated {
            self.rotated = false;

            if self.events_enabled {
                self.event_queue
                    .push(EventKind::DisplayRotated {
                        id: self.display_id.unwrap(),
                        new_orientation: self.orientation,
                    })
                    .unwrap(); // TODO: Add error handling
            }
        }

        self.events_enabled = true;
    }

    fn scale(&mut self, _: i32) {}

    fn name(&mut self, name: &CStr) {
        if self.name.len() == 0 {
            self.name = name.to_string_lossy().into_owned();
        }

        if self.id.len() == 0 {
            self.id = name.to_string_lossy().into_owned();
        }
    }

    fn description(&mut self, description: &CStr) {
        self.name = description.to_string_lossy().into_owned();
    }
}
