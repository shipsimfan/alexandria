use crate::math::Matrix4x4;

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix4x4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Matrix4x4))
            .field("r0", &self.r0)
            .field("r1", &self.r1)
            .field("r2", &self.r2)
            .field("r3", &self.r3)
            .finish()
    }
}
