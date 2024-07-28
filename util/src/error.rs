/// Common error trait
pub trait Error: 'static + std::error::Error {
    /// The title to be displayed in a message box
    fn title(&self) -> &'static str;
}

impl<T: Error> From<T> for Box<dyn Error> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
