use win32::ATOM;

mod deref;
mod drop;
mod register;

/// The window class the game window belongs to
pub(in crate::window) struct WindowClass {
    class: ATOM,
}
