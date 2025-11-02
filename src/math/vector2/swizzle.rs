use crate::math::{Vector2, Vector3};

impl<T> Vector2<T> {
    /// Gets this [`Vector2`] as a [`Vector2`] with values `(x, y)`
    pub const fn xy(self) -> Vector2<T> {
        self
    }

    /// Gets this [`Vector2`] as a [`Vector2`] with values `(y, x)`
    pub fn yx(self) -> Vector2<T> {
        Vector2::new(self.y, self.x)
    }
}

impl<T: Clone> Vector2<T> {
    /// Gets this [`Vector2`] as a [`Vector2`] with values `(x, x)`
    pub fn xx(self) -> Vector2<T> {
        Vector2::new(self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector2`] with values `(y, y)`
    pub fn yy(self) -> Vector2<T> {
        Vector2::new(self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, x, x)`
    pub fn xxx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, x, y)`
    pub fn xxy(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x, self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, y, x)`
    pub fn xyx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.y, self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, y, y)`
    pub fn xyy(self) -> Vector3<T> {
        Vector3::new(self.x, self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, x, x)`
    pub fn yxx(self) -> Vector3<T> {
        Vector3::new(self.y, self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, x, y)`
    pub fn yxy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.x, self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, y, x)`
    pub fn yyx(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y, self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, y, y)`
    pub fn yyy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y.clone(), self.y)
    }
}
