use crate::{
    math::{Rational, Recti, Vector2u},
    window::{DisplayOrientation, display::DisplayInner},
};

impl<UserEvent: 'static + Send> DisplayInner<UserEvent> {
    /// Get the rectangle describing the area of this display covers
    pub fn rect(&self) -> Recti {
        match self {
            DisplayInner::Wayland(wayland) => wayland.rect(),
            DisplayInner::X11 => todo!(),
        }
    }

    /// Get the rectangle describing the work area of this display
    pub fn work_area(&self) -> Recti {
        match self {
            DisplayInner::Wayland(wayland) => wayland.work_area(),
            DisplayInner::X11 => todo!(),
        }
    }

    /// Get the current refresh rate
    pub fn refresh_rate(&self) -> Rational {
        match self {
            DisplayInner::Wayland(wayland) => wayland.refresh_rate(),
            DisplayInner::X11 => todo!(),
        }
    }

    /// Get the scale factor for UI
    pub fn content_scale(&self) -> f32 {
        match self {
            DisplayInner::Wayland(wayland) => wayland.content_scale(),
            DisplayInner::X11 => todo!(),
        }
    }

    /// Get the physical of the display in millimeters
    pub fn physical_size(&self) -> Option<Vector2u> {
        match self {
            DisplayInner::Wayland(wayland) => wayland.physical_size(),
            DisplayInner::X11 => todo!(),
        }
    }

    /// Get the current orientation of the display
    pub fn current_orientation(&self) -> DisplayOrientation {
        match self {
            DisplayInner::Wayland(wayland) => wayland.current_orientation(),
            DisplayInner::X11 => todo!(),
        }
    }

    /// Is this display the primary display?
    pub fn is_primary(&self) -> bool {
        match self {
            DisplayInner::Wayland(wayland) => wayland.is_primary(),
            DisplayInner::X11 => todo!(),
        }
    }

    /// Get the name of this display
    pub fn name(&self) -> &str {
        match self {
            DisplayInner::Wayland(wayland) => wayland.name(),
            DisplayInner::X11 => todo!(),
        }
    }

    /// Get the id of this display
    pub fn id(&self) -> &str {
        match self {
            DisplayInner::Wayland(wayland) => wayland.id(),
            DisplayInner::X11 => todo!(),
        }
    }

    /// Get the name of this display that the registry provided
    pub(in crate::window) fn wayland_name(&self) -> Option<u32> {
        match self {
            DisplayInner::Wayland(wayland) => Some(wayland.wayland_name()),
            DisplayInner::X11 => None,
        }
    }
}
