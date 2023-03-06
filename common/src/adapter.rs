pub trait Adapter: Sized + 'static {
    fn name(&self) -> &str;
}
