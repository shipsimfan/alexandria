use re_enumerate_displays::re_enumerate_displays;

mod new;
mod pump;
mod re_enumerate_displays;
mod window_proc;

/// The window procedure for the message only window
pub(in crate::window::subsystem::windows) struct MessageOnlyWndProc {
    /// Should the displays be enumerated?
    enumerate_displays: bool,

    /// Should the displays refresh their DPIs?
    refresh_dpi: bool,
}
