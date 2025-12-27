use crate::Matrix3x3;
use std::{
    marker::Destruct,
    ops::{Add, Mul, MulAssign},
};

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    Mul for Matrix3x3<T>
{
    type Output = Matrix3x3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3x3::new_rows(
            self.r0 * rhs.clone(),
            self.r1 * rhs.clone(),
            self.r2 * rhs.clone(),
        )
    }
}

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    MulAssign for Matrix3x3<T>
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix3x3f;

    macro_rules! matrix_mul_tests {
        [$(
            $test_name: ident: ($i1: expr, $i2: expr) -> $o: expr,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Matrix3x3f = Matrix3x3f::from_row_array($i1);
                const INPUT2: Matrix3x3f = Matrix3x3f::from_row_array($i2);
                const OUTPUT: Matrix3x3f = Matrix3x3f::from_row_array($o);

                let output = INPUT1 * INPUT2;

                assert!(output.approx_eq(OUTPUT, 1e-6), "matrix multiply failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    matrix_mul_tests![];
}
