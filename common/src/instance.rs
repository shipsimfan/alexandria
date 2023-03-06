pub trait Instance: Sized + 'static {
    type Error: std::error::Error;

    type Adapter: crate::Adapter;
    type AdapterIter<'a>: Iterator<Item = Result<Self::Adapter, Self::Error>>;

    fn new() -> Result<Self, Self::Error>;

    fn enum_adapters<'a>(&'a mut self) -> Result<Self::AdapterIter<'a>, Self::Error>;
    fn default_adapter(&mut self) -> Result<Self::Adapter, Self::Error>;
}
