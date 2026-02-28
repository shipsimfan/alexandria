use crate::{
    Id,
    math::{Rational, Recti, Vector2i},
    window::{Display, DisplayOrientation, Window},
};

mod from;

/// A specific event kind that can pushed into an [`EventQueue`](crate::EventQueue)
#[derive(Debug, Clone, PartialEq)]
pub enum EventKind<UserEvent: 'static + Send> {
    /*
     *** GENERAL EVENTS ***
     */
    /// The process has been requested to be shutdown
    Quit,

    /*
     *** DISPLAY EVENTS ***
     */
    /// A new [`Display`] was added
    DisplayAdded {
        /// The ID of the newly added [`Display`]
        id: Id<Display<'static, UserEvent>>,
    },

    /// A new [`Display`] was removed
    DisplayRemoved {
        /// The ID of the removed [`Display`]
        id: Id<Display<'static, UserEvent>>,
    },

    /// A [`Display`] was moved
    DisplayMoved {
        /// The ID of the moved [`Display`]
        id: Id<Display<'static, UserEvent>>,

        /// The new position of the [`Display`]
        new_position: Vector2i,
    },

    /// A [`Display`] was resized
    DisplayResized {
        /// The ID of the resized [`Display`]
        id: Id<Display<'static, UserEvent>>,
        /// The new size of the [`Display`]
        new_size: Vector2i,
    },

    /// A [`Display`]'s work area changed
    DisplayWorkAreaChanged {
        /// The ID of the changed [`Display`]
        id: Id<Display<'static, UserEvent>>,

        /// The new work area of the [`Display`]
        new_work_area: Recti,
    },

    /// A [`Display`]'s refresh rate changed
    DisplayRefreshRateChanged {
        /// The ID of the changed [`Display`]
        id: Id<Display<'static, UserEvent>>,

        /// The new refresh rate of the [`Display`]
        new_refresh_rate: Rational,
    },

    /// A [`Display`] was rotated
    DisplayRotated {
        /// The ID of the rotated [`Display`]
        id: Id<Display<'static, UserEvent>>,

        /// The new orientation of the [`Display`]
        new_orientation: DisplayOrientation,
    },

    /// A [`Display`]'s content scale changed
    DisplayContentScaleChanged {
        /// The ID of the changed [`Display`]
        id: Id<Display<'static, UserEvent>>,

        /// The new content scale of the [`Display`]
        new_content_scale: f32,
    },

    /*
     *** WINDOW EVENTS ***
     */
    /// A [`Window`] was requested to be closed
    WindowCloseRequest {
        /// The ID of the [`Window`] that is requesting to be closed
        id: Id<Window<UserEvent>>,
    },

    /*
     *** USER EVENT ***
     */
    /// An event defined by the user of Alexandria
    User(UserEvent),
}
