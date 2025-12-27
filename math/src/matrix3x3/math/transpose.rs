use crate::{Matrix3x3, Vector3};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Transpose this matrix about the diagonal
    pub const fn transpose(self) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_rows(
            Vector3::new(self.r0.x, self.r1.x, self.r2.x),
            Vector3::new(self.r0.y, self.r1.y, self.r2.y),
            Vector3::new(self.r0.z, self.r1.z, self.r2.z),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix3x3f;

    macro_rules! tranpose_tests {
        [$(
            $test_name: ident: ($i: expr) -> $o: expr,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix3x3f = Matrix3x3f::from_row_array($i);
                const OUTPUT: Matrix3x3f = Matrix3x3f::from_row_array($o);

                let output = INPUT.transpose();

                assert!(output.approx_eq(OUTPUT, 1e-6), "transpose failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    tranpose_tests![];
}
