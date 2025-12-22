use crate::{
    Vector2,
    number::{One, Sqrt, Zero},
};
use std::ops::{Add, Div, DivAssign, Mul, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + DivAssign
        + Sqrt
        + PartialOrd
        + Clone
        + Zero
        + One,
> Vector2<T>
{
    /// Interpolates linearly between two vectors, normalizing the result
    ///
    /// `t` will be clamped between 0 and 1
    pub fn nlerp(self, other: Self, t: T) -> Self {
        self.lerp(other, t).normalized()
    }

    /// Interpolates linearly between two vectors, normalizing the result
    ///
    /// `t` will not be clamped between 0 and 1
    pub fn nlerp_unclamped(self, other: Self, t: T) -> Self {
        self.lerp_unclamped(other, t).normalized()
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector2f;

    macro_rules! nlerp_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal),
                ($i2x: literal, $i2y: literal),
                $t: literal
                -> ($ox: literal, $oy: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector2f = Vector2f::new($i1x, $i1y);
                const INPUT2: Vector2f = Vector2f::new($i2x, $i2y);
                const OUTPUT: Vector2f = Vector2f::new($ox, $oy);
                const T: f32 = $t;

                let output = INPUT1.nlerp_unclamped(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "nlerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    nlerp_tests![
        nlerp_mid_orth: (1.0, 0.0), (0.0, 1.0), 0.5 -> (0.707106781, 0.707106781),
        nlerp_t0: (3.0, 4.0), (10.0, -2.0), 0.0 -> (0.600000000, 0.800000000),
        nlerp_t1: (3.0, 4.0), (10.0, -2.0), 1.0 -> (0.980580676, -0.196116135),
        nlerp_quarter: (1.0, 0.0), (0.0, 1.0), 0.25 -> (0.948683298, 0.316227766),
        nlerp_neg_mix: (-2.0, 1.0), (4.0, 3.0), 0.5 -> (0.447213595, 0.894427191),
        nlerp_biased: (2.0, 2.0), (4.0, 0.0), 0.75 -> (0.989949494, 0.141421356),
        nlerp_opposite_not_zero: (1.0, 1.0), (-1.0, 2.0), 0.5 -> (0.000000000, 1.000000000),
        nlerp_small_t: (5.0, -12.0), (8.0, 15.0), 0.1 -> (0.495132530, -0.868817459),
        nlerp_large_t: (5.0, -12.0), (8.0, 15.0), 0.9 -> (0.530618227, 0.847610935),
        nlerp_axis_weighted: (0.0, 2.0), (3.0, 0.0), 0.2 -> (0.351123442, 0.936329178),
    ];
}
