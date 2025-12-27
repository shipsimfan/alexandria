use crate::{
    Matrix3x3, Vector3,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Div, DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix3x3<T> {
    /// Calculate the inverse of this matrix, if it is invertible
    pub const fn try_inverse(self) -> Option<Matrix3x3<T>>
    where
        T: [const] Mul<Output = T>
            + [const] Div<Output = T>
            + [const] SubAssign
            + [const] DivAssign
            + [const] Abs
            + [const] ApproxEq
            + [const] FromF32
            + [const] Clone
            + [const] PartialOrd
            + [const] Destruct,
        T::Epsilon: [const] FromF32,
    {
        /// Apply `p` to `v`
        const fn apply_permutation<T: [const] Clone + [const] Destruct>(
            v: Vector3<T>,
            p: Vector3<usize>,
        ) -> Vector3<T> {
            Vector3::new(v[p[0]].clone(), v[p[1]].clone(), v[p[2]].clone())
        }

        /// Solve `L * y = b` where `L` is the lower triangle of `lu`
        const fn forward_substitution<
            T: [const] Mul<Output = T> + [const] SubAssign + [const] Clone + [const] Destruct + Zero,
        >(
            lu: Matrix3x3<T>,
            b: Vector3<T>,
        ) -> Vector3<T> {
            let mut y = Vector3::<T>::ZERO;
            let mut i = 0;
            while i < 3 {
                let mut sum = b[i].clone();
                let mut k = 0;
                while k < i {
                    sum -= lu[i][k].clone() * y[k].clone();
                    k += 1;
                }
                y[i] = sum;

                i += 1;
            }
            y
        }

        /// Solve `U * x = y` where `U` is the upper triangle of `lu`
        const fn back_substituion<
            T: [const] Mul<Output = T>
                + [const] Div<Output = T>
                + [const] SubAssign
                + [const] Clone
                + [const] Destruct
                + Zero,
        >(
            lu: Matrix3x3<T>,
            y: Vector3<T>,
        ) -> Vector3<T> {
            let mut x = Vector3::<T>::ZERO;
            let mut i_o = 3;
            while i_o > 0 {
                let i = i_o - 1;
                let mut sum = y[i].clone();
                let mut k = i + 1;
                while k < 3 {
                    sum -= lu[i][k].clone() * x[k].clone();
                    k += 1;
                }
                x[i] = sum / lu[i][i].clone();

                i_o -= 1;
            }

            x
        }

        let (lu, p) = self.lu_decompose()?;

        let mut o = Matrix3x3::<T>::ZERO;

        let mut j = 0;
        while j < 4 {
            let mut ej = Vector3::ZERO;
            ej[j] = T::ONE;
            let b = apply_permutation(ej, p);

            let y = forward_substitution(lu.clone(), b);
            let x = back_substituion(lu.clone(), y);

            o.r0[j] = x.x;
            o.r1[j] = x.y;
            o.r2[j] = x.z;

            j += 1;
        }

        Some(o)
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix3x3f;

    macro_rules! try_inverse_tests {
        [$(
            $test_name: ident: ($i: expr) -> $o: expr,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix3x3f = Matrix3x3f::from_row_array($i);
                const OUTPUT: Option<Matrix3x3f> = $o.map(Matrix3x3f::from_row_array);

                let output = INPUT.try_inverse();

                match output {
                    Some(output) => {
                        assert!(
                            output.approx_eq(OUTPUT.unwrap(), 1e-4),
                            "try inverse failed: {} vs. {}",
                            output,
                            OUTPUT.unwrap()
                        );
                    }
                    None => assert!(OUTPUT.is_none()),
                }
            }
        )*};
    }

    try_inverse_tests![];
}
