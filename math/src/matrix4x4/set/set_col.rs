use crate::{Matrix4x4, Vector4};

impl<T> Matrix4x4<T> {
    /// Set the column at `col` to `value`
    pub fn set_col(&mut self, col: usize, value: Vector4<T>) {
        match col {
            0 => {
                self.r0.x = value.x;
                self.r1.x = value.y;
                self.r2.x = value.z;
                self.r3.x = value.w;
            }
            1 => {
                self.r0.y = value.x;
                self.r1.y = value.y;
                self.r2.y = value.z;
                self.r3.y = value.w;
            }
            2 => {
                self.r0.z = value.x;
                self.r1.z = value.y;
                self.r2.z = value.z;
                self.r3.z = value.w;
            }
            3 => {
                self.r0.w = value.x;
                self.r1.w = value.y;
                self.r2.w = value.z;
                self.r3.w = value.w;
            }
            _ => panic!("index out of bounds: the len is 4 but the index is {}", col),
        }
    }
}
