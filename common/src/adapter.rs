pub trait Adapter: Sized + 'static {
    type Error: std::error::Error;

    type Display: crate::Display;
    type DisplayIter<'a>: Iterator<Item = Result<Self::Display, Self::Error>>;

    fn name(&self) -> &str;

    fn enum_displays<'a>(&'a mut self) -> Result<Self::DisplayIter<'a>, Self::Error>;
    fn default_display(&mut self) -> Result<Self::Display, Self::Error>;
}
