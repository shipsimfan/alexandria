use crate::Quaternion;

impl<T: std::fmt::Debug> std::fmt::Debug for Quaternion<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Quaternion))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}
