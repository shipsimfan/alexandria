use crate::math::{Matrix3x3, Vector3};

impl<T> Matrix3x3<T> {
    /// Set the column at `col` to `value`
    pub fn set_col(&mut self, col: usize, value: Vector3<T>) {
        match col {
            0 => {
                self.r0.x = value.x;
                self.r1.x = value.y;
                self.r2.x = value.z;
            }
            1 => {
                self.r0.y = value.x;
                self.r1.y = value.y;
                self.r2.y = value.z;
            }
            2 => {
                self.r0.z = value.x;
                self.r1.z = value.y;
                self.r2.z = value.z;
            }
            _ => panic!("index out of bounds: the len is 3 but the index is {}", col),
        }
    }
}
