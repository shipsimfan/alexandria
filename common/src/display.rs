pub trait Display: Sized + 'static {
    fn name(&self) -> &str;
}
