use crate::{Matrix3x3, Vector3};
use std::{
    marker::Destruct,
    ops::{Add, Mul, MulAssign},
};

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    Mul<Vector3<T>> for Matrix3x3<T>
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector3::new(
            self.r0.dot(rhs.clone()),
            self.r1.dot(rhs.clone()),
            self.r2.dot(rhs),
        )
    }
}

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    Mul<Matrix3x3<T>> for Vector3<T>
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Matrix3x3<T>) -> Self::Output {
        rhs.transpose() * self
    }
}

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    MulAssign<Matrix3x3<T>> for Vector3<T>
{
    fn mul_assign(&mut self, rhs: Matrix3x3<T>) {
        *self = self.clone() * rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix3x3f, Vector3f};

    macro_rules! right_vec_mul_tests {
        [$(
            $test_name: ident: ($m: expr, [$vx: literal, $vy: literal, $vz: literal, $vw: literal]) -> [$ox: literal, $oy: literal, $oz: literal, $ow: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const MATRIX: Matrix3x3f = Matrix3x3f::from_row_array($m);
                const VECTOR: Vector3f = Vector3f::new($vx, $vy, $vz, $vw);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz, $ow);

                let output = MATRIX * VECTOR;

                assert!(output.approx_eq(OUTPUT, 1e-6), "right vector multiply failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    macro_rules! left_vec_mul_tests {
        [$(
            $test_name: ident: ([$vx: literal, $vy: literal, $vz: literal, $vw: literal], $m: expr) -> [$ox: literal, $oy: literal, $oz: literal, $ow: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const MATRIX: Matrix3x3f = Matrix3x3f::from_row_array($m);
                const VECTOR: Vector3f = Vector3f::new($vx, $vy, $vz, $vw);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz, $ow);

                let output = VECTOR * MATRIX;

                assert!(output.approx_eq(OUTPUT, 1e-6), "left vector multiply failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    right_vec_mul_tests![];

    left_vec_mul_tests![];
}
