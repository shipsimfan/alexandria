use crate::math::{
    Matrix4x4, Vector4,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Div, DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix4x4<T> {
    /// Calculate the inverse of this matrix, if it is invertible
    pub const fn try_inverse(self) -> Option<Matrix4x4<T>>
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
            v: Vector4<T>,
            p: Vector4<usize>,
        ) -> Vector4<T> {
            Vector4::new(
                v[p[0]].clone(),
                v[p[1]].clone(),
                v[p[2]].clone(),
                v[p[3]].clone(),
            )
        }

        /// Solve `L * y = b` where `L` is the lower triangle of `lu`
        const fn forward_substitution<
            T: [const] Mul<Output = T> + [const] SubAssign + [const] Clone + [const] Destruct + Zero,
        >(
            lu: Matrix4x4<T>,
            b: Vector4<T>,
        ) -> Vector4<T> {
            let mut y = Vector4::<T>::ZERO;
            let mut i = 0;
            while i < 4 {
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
            lu: Matrix4x4<T>,
            y: Vector4<T>,
        ) -> Vector4<T> {
            let mut x = Vector4::<T>::ZERO;
            let mut i_o = 4;
            while i_o > 0 {
                let i = i_o - 1;
                let mut sum = y[i].clone();
                let mut k = i + 1;
                while k < 4 {
                    sum -= lu[i][k].clone() * x[k].clone();
                    k += 1;
                }
                x[i] = sum / lu[i][i].clone();

                i_o -= 1;
            }

            x
        }

        let (lu, p) = self.lu_decompose()?;

        let mut o = Matrix4x4::<T>::ZERO;

        let mut j = 0;
        while j < 4 {
            let mut ej = Vector4::ZERO;
            ej[j] = T::ONE;
            let b = apply_permutation(ej, p);

            let y = forward_substitution(lu.clone(), b);
            let x = back_substituion(lu.clone(), y);

            o.r0[j] = x.x;
            o.r1[j] = x.y;
            o.r2[j] = x.z;
            o.r3[j] = x.w;

            j += 1;
        }

        Some(o)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Matrix4x4f;

    macro_rules! try_inverse_tests {
        [$(
            $test_name: ident: ($i: expr) -> $o: expr,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix4x4f = Matrix4x4f::from_row_array($i);
                const OUTPUT: Option<Matrix4x4f> = $o.map(Matrix4x4f::from_row_array);

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

    try_inverse_tests![
        try_inverse_identity: (
          [[1.0, 0.0, 0.0, 0.0],
           [0.0, 1.0, 0.0, 0.0],
           [0.0, 0.0, 1.0, 0.0],
           [0.0, 0.0, 0.0, 1.0]]
        ) -> Some(
          [[1.0, 0.0, 0.0, 0.0],
           [0.0, 1.0, 0.0, 0.0],
           [0.0, 0.0, 1.0, 0.0],
           [0.0, 0.0, 0.0, 1.0]]
        ),

        try_inverse_translation_affine: (
          [[1.0, 0.0, 0.0,  3.0],
           [0.0, 1.0, 0.0, -2.0],
           [0.0, 0.0, 1.0,  5.0],
           [0.0, 0.0, 0.0,  1.0]]
        ) -> Some(
          [[1.0, 0.0, 0.0, -3.0],
           [0.0, 1.0, 0.0,  2.0],
           [0.0, 0.0, 1.0, -5.0],
           [0.0, 0.0, 0.0,  1.0]]
        ),

        try_inverse_nonuniform_scale: (
          [[ 2.0, 0.0, 0.0, 0.0],
           [ 0.0,-4.0, 0.0, 0.0],
           [ 0.0, 0.0, 0.5, 0.0],
           [ 0.0, 0.0, 0.0, 1.0]]
        ) -> Some(
          [[ 0.5, 0.0, 0.0, 0.0],
           [ 0.0,-0.25,0.0, 0.0],
           [ 0.0, 0.0, 2.0, 0.0],
           [ 0.0, 0.0, 0.0, 1.0]]
        ),

        try_inverse_rot_z_90_affine: (
          [[ 0.0,-1.0, 0.0, 0.0],
           [ 1.0, 0.0, 0.0, 0.0],
           [ 0.0, 0.0, 1.0, 0.0],
           [ 0.0, 0.0, 0.0, 1.0]]
        ) -> Some(
          [[ 0.0, 1.0, 0.0, 0.0],
           [-1.0, 0.0, 0.0, 0.0],
           [ 0.0, 0.0, 1.0, 0.0],
           [ 0.0, 0.0, 0.0, 1.0]]
        ),

        try_inverse_shear_x_by_yz_affine: (
          [[1.0, 2.0,-1.0, 0.0],
           [0.0, 1.0, 0.0, 0.0],
           [0.0, 0.0, 1.0, 0.0],
           [0.0, 0.0, 0.0, 1.0]]
        ) -> Some(
          [[1.0,-2.0, 1.0, 0.0],
           [0.0, 1.0, 0.0, 0.0],
           [0.0, 0.0, 1.0, 0.0],
           [0.0, 0.0, 0.0, 1.0]]
        ),

        try_inverse_general_full_4x4: (
          [[2.0, 1.0, 0.0, 3.0],
           [0.0, 1.0, 4.0, 2.0],
           [1.0, 0.0, 1.0, 1.0],
           [0.0, 2.0, 0.0, 1.0]]
        ) -> Some(
          [[-0.5, -0.5,  2.0,  0.5],
           [-0.4, -0.2,  0.8,  0.8],
           [-0.3,  0.1,  0.6,  0.1],
           [ 0.8,  0.4, -1.6, -0.6]]
        ),

        try_inverse_tiny_last_diagonal: (
          [[1.0, 0.0, 0.0,   0.0],
           [0.0, 1.0, 0.0,   0.0],
           [0.0, 0.0, 1.0,   0.0],
           [0.0, 0.0, 0.0, 0.001]]
        ) -> Some(
          [[1.0, 0.0, 0.0,    0.0],
           [0.0, 1.0, 0.0,    0.0],
           [0.0, 0.0, 1.0,    0.0],
           [0.0, 0.0, 0.0, 1000.0]]
        ),

        try_inverse_singular_duplicate_row: (
          [[1.0, 2.0, 3.0, 4.0],
           [1.0, 2.0, 3.0, 4.0],
           [0.0, 1.0, 0.0, 1.0],
           [0.0, 0.0, 1.0, 0.0]]
        ) -> None,

        try_inverse_singular_zero_scale_axis: (
          [[1.0, 0.0, 0.0, 0.0],
           [0.0, 0.0, 0.0, 0.0],
           [0.0, 0.0, 1.0, 0.0],
           [0.0, 0.0, 0.0, 1.0]]
        ) -> None,
    ];
}
