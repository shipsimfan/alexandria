use win32::Interface;

pub struct Adapter {
    inner: win32::IDXGIAdapter4,

    name: String,
}

impl Adapter {
    pub(crate) fn new(mut inner: win32::IDXGIAdapter1) -> Result<Self, crate::Error> {
        let mut inner: win32::IDXGIAdapter4 = inner.query_interface()?;

        let desc = inner.get_desc3()?;

        let name = String::from_utf16(desc.description()).unwrap();

        Ok(Adapter { inner, name })
    }
}

impl common::Adapter for Adapter {
    fn name(&self) -> &str {
        &self.name
    }
}
