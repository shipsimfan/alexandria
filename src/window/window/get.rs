use crate::{
    math::{Recti, Vector2i},
    window::Window,
};

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Get the position and size of the window's client area, in screen coordinates
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn rect_opt(&self) -> Option<Recti> {
        self.with_inner(|inner| inner.rect())
    }

    /// Get the position and size of the window's client area, in screen coordinates
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn rect(&self) -> Recti {
        self.rect_opt().expect("window has been destroyed")
    }

    /// Get the position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn position_opt(&self) -> Option<Vector2i> {
        self.rect_opt().map(|rect| rect.position)
    }

    /// Get the position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn position(&self) -> Vector2i {
        self.position_opt().expect("window has been destroyed")
    }

    /// Get the x-position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn x_opt(&self) -> Option<i32> {
        self.position_opt().map(|position| position.x)
    }

    /// Get the x-position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn x(&self) -> i32 {
        self.x_opt().expect("window has been destroyed")
    }

    /// Get the y-position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn y_opt(&self) -> Option<i32> {
        self.position_opt().map(|position| position.y)
    }

    /// Get the y-position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn y(&self) -> i32 {
        self.y_opt().expect("window has been destroyed")
    }

    /// Get the size of the window's client area
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn size_opt(&self) -> Option<Vector2i> {
        self.rect_opt().map(|rect| rect.size)
    }

    /// Get the size of the window's client area
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn size(&self) -> Vector2i {
        self.size_opt().expect("window has been destroyed")
    }

    /// Get the width of the window's client area
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn width_opt(&self) -> Option<i32> {
        self.size_opt().map(|size| size.x)
    }

    /// Get the width of the window's client area
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn width(&self) -> i32 {
        self.width_opt().expect("window has been destroyed")
    }

    /// Get the height of the window's client area
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn height_opt(&self) -> Option<i32> {
        self.size_opt().map(|size| size.y)
    }

    /// Get the height of the window's client area
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn height(&self) -> i32 {
        self.height_opt().expect("window has been destroyed")
    }
}
