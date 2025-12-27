use crate::{
    Quaternion,
    number::{One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

const fn lerp_unclamped<
    T: [const] Add<Output = T> + [const] Sub<Output = T> + [const] Mul<Output = T> + [const] Clone,
>(
    a: T,
    b: T,
    t: T,
) -> T {
    a.clone() + t * (b - a)
}

impl<T> Quaternion<T> {
    /// Interpolates linearly between two vectors, clamping `t` between 0 and 1
    pub const fn lerp(self, other: Self, t: T) -> Self
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Clone
            + [const] PartialOrd
            + [const] Destruct
            + Zero
            + One,
    {
        self.lerp_unclamped(
            other,
            if t < T::ZERO {
                T::ZERO
            } else if t > T::ONE {
                T::ONE
            } else {
                t
            },
        )
    }

    /// Interpolates linearly between two vectors, without clamping `t` between 0 and 1
    pub const fn lerp_unclamped(self, other: Self, t: T) -> Self
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        self.zip2(other, Quaternion::splat(t), lerp_unclamped)
    }
}

#[cfg(test)]
mod tests {
    use crate::Quaternionf;

    macro_rules! lerp_tests {
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

                let output = INPUT1.lerp_unclamped(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "lerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    lerp_tests![
        lerp_t0_returns_a: (1.0, -2.0, 3.0, -4.0), (9.0, 8.0, 7.0, 6.0), 0.0 -> (1.0, -2.0, 3.0, -4.0),
        lerp_t1_returns_b: (1.0, -2.0, 3.0, -4.0), (9.0, 8.0, 7.0, 6.0), 1.0 -> (9.0, 8.0, 7.0, 6.0),

        lerp_midpoint_t_half: (0.0, 2.0, -4.0, 6.0), (8.0, -2.0, 4.0, -6.0), 0.5 -> (4.0, 0.0, 0.0, 0.0),
        lerp_quarter_t_0_25: (0.0, 0.0, 0.0, 0.0), (8.0, 4.0, -8.0, 12.0), 0.25 -> (2.0, 1.0, -2.0, 3.0),
        lerp_three_quarters_t_0_75: (0.0, 0.0, 0.0, 0.0), (8.0, 4.0, -8.0, 12.0), 0.75 -> (6.0, 3.0, -6.0, 9.0),

        lerp_negative_to_positive: (-8.0, -4.0, -2.0, -1.0), (8.0, 4.0, 2.0, 1.0), 0.5 -> (0.0, 0.0, 0.0, 0.0),
        lerp_mixed_signs_eighth: (16.0, -16.0, 8.0, -8.0), (0.0, 0.0, 0.0, 0.0), 0.125 -> (14.0, -14.0, 7.0, -7.0),

        lerp_a_equals_b_any_t: (3.0, -5.0, 7.0, -9.0), (3.0, -5.0, 7.0, -9.0), 0.3 -> (3.0, -5.0, 7.0, -9.0),
        lerp_zero_vector_to_values_half: (0.0, 0.0, 0.0, 0.0), (2.0, -4.0, 6.0, -8.0), 0.5 -> (1.0, -2.0, 3.0, -4.0),

        lerp_extrapolate_t_minus_half: (4.0, -2.0, 0.0, 1.0), (8.0, 2.0, 4.0, 5.0), -0.5 -> (2.0, -4.0, -2.0, -1.0),
        lerp_extrapolate_t_1_5: (4.0, -2.0, 0.0, 1.0), (8.0, 2.0, 4.0, 5.0), 1.5 -> (10.0, 4.0, 6.0, 7.0),

        lerp_reverse_direction_t_half: (10.0, 8.0, 6.0, 4.0), (2.0, 4.0, 6.0, 8.0), 0.5 -> (6.0, 6.0, 6.0, 6.0),
        lerp_reverse_direction_quarter: (10.0, 8.0, 6.0, 4.0), (2.0, 4.0, 6.0, 8.0), 0.25 -> (8.0, 7.0, 6.0, 5.0),

        lerp_nonuniform_components_half: (1.0, 2.0, 4.0, 8.0), (9.0, 6.0, 0.0, -8.0), 0.5 -> (5.0, 4.0, 2.0, 0.0),
        lerp_nonuniform_components_eighth: (1.0, 2.0, 4.0, 8.0), (9.0, 6.0, 0.0, -8.0), 0.125 -> (2.0, 2.5, 3.5, 6.0),

        lerp_small_integers_t_half: (-1.0, 0.0, 1.0, 2.0), (3.0, 2.0, 1.0, 0.0), 0.5 -> (1.0, 1.0, 1.0, 1.0),
        lerp_small_integers_t_0_75: (-1.0, 0.0, 1.0, 2.0), (3.0, 2.0, 1.0, 0.0), 0.75 -> (2.0, 1.5, 1.0, 0.5),

        lerp_large_magnitude_safe_half: (1000000.0, -1000000.0, 500000.0, -500000.0), (1000008.0, -999992.0, 499992.0, -499992.0), 0.5 -> (1000004.0, -999996.0, 499996.0, -499996.0),
        lerp_large_magnitude_safe_quarter: (1000000.0, -1000000.0, 500000.0, -500000.0), (1000008.0, -999992.0, 499992.0, -499992.0), 0.25 -> (1000002.0, -999998.0, 499998.0, -499998.0),
    ];
}
