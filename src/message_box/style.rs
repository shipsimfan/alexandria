use win32::{MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, UINT};

/// A style a message box can be displayed with
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxStyle {
    /// A stop sign icon is used for the message box
    Error,

    /// An exclamation point is used for the message box
    Warning,

    /// An icon of a lowercase 'i' in a circle is used for the message box
    Information,
}

impl MessageBoxStyle {
    /// Get the win32 style for this
    pub(in crate::message_box) fn to_win32(&self) -> UINT {
        match self {
            MessageBoxStyle::Error => MB_ICONERROR,
            MessageBoxStyle::Warning => MB_ICONWARNING,
            MessageBoxStyle::Information => MB_ICONINFORMATION,
        }
    }
}
