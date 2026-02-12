use crate::window::{Display, display::DisplayInner};

impl Display {
    /// Create a new [`Display`]
    fn new(inner: DisplayInner) -> Display {
        Display { inner }
    }
}
