pub trait DisplayMode {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn refresh_rate(&self) -> f32;
}

pub trait Display: Sized + 'static {
    type DisplayMode: DisplayMode;
    type DisplayModeIter<'a>: Iterator<Item = &'a Self::DisplayMode>;

    fn name(&self) -> &str;
    fn display_modes<'a>(&'a self) -> Self::DisplayModeIter<'a>;
}
