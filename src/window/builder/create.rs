use crate::{Adapter, Error, Result, Window, WindowBuilder};

/// "Owned or borrowed"
enum Orb<'a, T> {
    Borrowed(&'a mut T),
    Owned(T),
}

impl<'a, T> AsMut<T> for Orb<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Orb::Borrowed(borrowed) => borrowed,
            Orb::Owned(owned) => owned,
        }
    }
}

impl<'a> WindowBuilder<'a> {
    /// Create a new [`Window`] with the provided settings
    pub fn create(self) -> Result<Box<Window>> {
        let mut title: Vec<_> = self.title.encode_utf16().collect();
        title.push(0);

        let mut adapter = match self.adapter {
            Some(adapter) => Orb::Borrowed(adapter),
            None => {
                let mut adapters = Adapter::enumerate()?;
                if adapters.len() == 0 {
                    return Err(Error::NoValidAdapter);
                }
                Orb::Owned(adapters.swap_remove(0))
            }
        };

        Window::new(
            &title,
            self.x,
            self.y,
            self.width,
            self.height,
            self.vsync,
            self.display_mode,
            adapter.as_mut(),
        )
    }
}
