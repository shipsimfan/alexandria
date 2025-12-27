use crate::Matrix3x3;

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix3x3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Matrix3x3))
            .field("r0", &self.r0)
            .field("r1", &self.r1)
            .field("r2", &self.r2)
            .finish()
    }
}
