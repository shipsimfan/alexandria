use crate::{Matrix3x3, Vector3, number::One};
use std::{
    marker::Destruct,
    ops::{Add, Div, Mul},
};

impl<T: One> Matrix3x3<T> {
    /// Transforms `p` using this matrix, including translation
    pub const fn transform_point(self, p: Vector3<T>) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Mul<Output = T>
            + [const] Div<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        self * p
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix3x3f, Vector3f};

    macro_rules! transform_tests {
        [$(
            $test_name: ident: ($m: expr, [$vx: literal, $vy: literal, $vz: literal]) -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const MATRIX: Matrix3x3f = Matrix3x3f::from_row_array($m);
                const VECTOR: Vector3f = Vector3f::new($vx, $vy, $vz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let output = MATRIX.transform_point(VECTOR);

                assert!(output.approx_eq(OUTPUT, 1e-6), "transform failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    transform_tests![];
}
