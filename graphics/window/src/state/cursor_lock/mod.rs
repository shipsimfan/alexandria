mod as_str;
mod default;
mod display;

/// How the cursor can be locked to the window
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorLock {
    /// The cursor is free to move anywhere on the screen
    Unlocked,

    /// The cursor must remain within the window, but can move freely within
    Locked,
}
