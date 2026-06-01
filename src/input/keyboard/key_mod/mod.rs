mod get;
mod modify;

/// The active modifier keys
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyMod {
    /// Is the left shift key active?
    pub left_shift: bool,

    /// Is the right shift key active?
    pub right_shift: bool,

    /// Is the left control key active?
    pub left_control: bool,

    /// Is the right control key active?
    pub right_control: bool,

    /// Is the left alt key active?
    pub left_alt: bool,

    /// Is the right alt key active?
    pub right_alt: bool,

    /// Is the left super key active?
    pub left_super: bool,

    /// Is the right super key active?
    pub right_super: bool,

    /// Is the caps lock active?
    pub caps_lock: bool,

    /// Is the num lock active?
    pub num_lock: bool,

    /// Is the scroll lock active?
    pub scroll_lock: bool,
}
