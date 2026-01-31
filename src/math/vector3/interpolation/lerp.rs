use crate::math::{
    Vector3,
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

impl<T> Vector3<T> {
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
        self.zip2(other, Vector3::splat(t), lerp_unclamped)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector3f;

    macro_rules! lerp_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal, $i1z: literal),
                ($i2x: literal, $i2y: literal, $i2z: literal),
                $t: literal
                -> ($ox: literal, $oy: literal, $oz: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector3f = Vector3f::new($i1x, $i1y, $i1z);
                const INPUT2: Vector3f = Vector3f::new($i2x, $i2y, $i2z);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);
                const T: f32 = $t;

                let output = INPUT1.lerp_unclamped(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "lerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    lerp_tests![
        lerp_zero_t: (1.25, -2.5, 3.75), (10.0, 20.0, -30.0), 0.0 -> (1.25, -2.5, 3.75),
        lerp_one_t: (-4.0, 5.5, 6.25), (7.0, -8.5, 9.75), 1.0 -> (7.0, -8.5, 9.75),
        lerp_halfway: (0.0, 0.0, 0.0), (2.0, -4.0, 6.0), 0.5 -> (1.0, -2.0, 3.0),
        lerp_quarter: (8.0, -8.0, 4.0), (0.0, 0.0, 12.0), 0.25 -> (6.0, -6.0, 6.0),
        lerp_three_quarters: (-2.0, 6.0, 10.0), (6.0, -2.0, 2.0), 0.75 -> (4.0, 0.0, 4.0),
        lerp_negative_t: (1.0, 2.0, 3.0), (4.0, 6.0, 8.0), -0.5 -> (-0.5, 0.0, 0.5),
        lerp_t_gt_one: (-1.0, 0.5, 2.0), (3.0, -1.5, -2.0), 1.5 -> (5.0, -2.5, -4.0),
        lerp_mixed_signs: (-3.5, 2.25, -1.75), (1.5, -5.75, 6.25), 0.2 -> (-2.5, 0.65, -0.15),
        lerp_large_values: (100000.0, -200000.0, 300000.0), (-100000.0, 400000.0, -500000.0), 0.1 -> (80000.0, -140000.0, 220000.0),
        lerp_small_values: (0.001, -0.002, 0.003), (0.004, 0.006, -0.009), 0.25 -> (0.00175, 0.0, 0.0),
        lerp_same_points: (5.0, -7.0, 9.0), (5.0, -7.0, 9.0), 0.37 -> (5.0, -7.0, 9.0),
        lerp_nontrivial_fraction: (-12.5, 7.0, 0.5), (2.5, -13.0, 10.5), 0.3 -> (-8.0, 1.0, 3.5),
    ];
}
