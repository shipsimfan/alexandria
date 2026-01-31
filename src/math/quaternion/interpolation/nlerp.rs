use crate::math::{
    Quaternion,
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
> Quaternion<T>
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
    use crate::math::Quaternionf;

    macro_rules! nlerp_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal, $i1z: literal, $i1w: literal),
                ($i2x: literal, $i2y: literal, $i2z: literal, $i2w: literal),
                $t: literal
                -> ($ox: literal, $oy: literal, $oz: literal, $ow: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Quaternionf = Quaternionf::new($i1x, $i1y, $i1z, $i1w);
                const INPUT2: Quaternionf = Quaternionf::new($i2x, $i2y, $i2z, $i2w);
                const OUTPUT: Quaternionf = Quaternionf::new($ox, $oy, $oz, $ow);
                const T: f32 = $t;

                let output = INPUT1.nlerp_unclamped(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "nlerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    nlerp_tests![
        nlerp_orthogonal_half: (1.0, 0.0, 0.0, 0.0), (0.0, 1.0, 0.0, 0.0), 0.5 -> (0.707106781, 0.707106781, 0.0, 0.0),
        nlerp_scaled_axes_half: (2.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 2.0), 0.5 -> (0.707106781, 0.0, 0.0, 0.707106781),
        nlerp_swap_half: (1.0, 2.0, 3.0, 4.0), (4.0, 3.0, 2.0, 1.0), 0.5 -> (0.5, 0.5, 0.5, 0.5),

        nlerp_nonunit_t0: (3.0, 0.0, 0.0, 0.0), (0.0, 4.0, 0.0, 0.0), 0.0 -> (1.0, 0.0, 0.0, 0.0),
        nlerp_nonunit_t1: (3.0, 0.0, 0.0, 0.0), (0.0, 4.0, 0.0, 0.0), 1.0 -> (0.0, 1.0, 0.0, 0.0),
        nlerp_nonunit_quarter: (3.0, 0.0, 0.0, 0.0), (0.0, 4.0, 0.0, 0.0), 0.25 -> (0.913811549, 0.406138466, 0.0, 0.0),

        nlerp_opposite_quarter: (1.0, 0.0, 0.0, 0.0), (-1.0, 0.0, 0.0, 0.0), 0.25 -> (1.0, 0.0, 0.0, 0.0),
        nlerp_opposite_three_quarter: (1.0, 0.0, 0.0, 0.0), (-1.0, 0.0, 0.0, 0.0), 0.75 -> (-1.0, 0.0, 0.0, 0.0),

        nlerp_negative_t: (1.0, 1.0, 0.0, 0.0), (1.0, 0.0, 1.0, 0.0), -0.5 -> (0.534522484, 0.801783726, -0.267261242, 0.0),
        nlerp_t_gt1: (0.0, 1.0, 1.0, 0.0), (1.0, 0.0, 1.0, 0.0), 1.5 -> (0.801783726, -0.267261242, 0.534522484, 0.0),

        nlerp_mixed_half: (1.0, -1.0, 2.0, -2.0), (-2.0, 2.0, -1.0, 1.0), 0.5 -> (-0.5, 0.5, 0.5, -0.5),
        nlerp_w_only: (0.0, 0.0, 0.0, 5.0), (1.0, 0.0, 0.0, 0.0), 0.2 -> (0.049937617, 0.0, 0.0, 0.998752339),
    ];
}
