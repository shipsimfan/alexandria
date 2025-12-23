use crate::{
    Vector2,
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

impl<T> Vector2<T> {
    /// Interpolates linearly between two vectors, clamping `t` between 0 and 1
    pub const fn lerp(self, other: Vector2<T>, t: T) -> Vector2<T>
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
    pub const fn lerp_unclamped(self, other: Vector2<T>, t: T) -> Vector2<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        self.zip2(other, Vector2::splat(t), lerp_unclamped)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector2f;

    macro_rules! lerp_tests {
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

                let output = INPUT1.lerp_unclamped(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "lerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    lerp_tests![
        lerp_t0_returns_a: (1.0, 2.0), (9.0, 10.0), 0.0 -> (1.0, 2.0),
        lerp_t1_returns_b: (1.0, 2.0), (9.0, 10.0), 1.0 -> (9.0, 10.0),
        lerp_halfway_midpoint: (0.0, 0.0), (8.0, -4.0), 0.5 -> (4.0, -2.0),
        lerp_quarter_step: (-4.0, 12.0), (4.0, 4.0), 0.25 -> (-2.0, 10.0),
        lerp_three_quarters_step: (2.0, -6.0), (10.0, 2.0), 0.75 -> (8.0, 0.0),
        lerp_extrapolate_t2: (3.0, -1.0), (7.0, 5.0), 2.0 -> (11.0, 11.0),
        lerp_extrapolate_tneg1: (3.0, -1.0), (7.0, 5.0), -1.0 -> (-1.0, -7.0),
        lerp_same_points_any_t: (6.0, -3.0), (6.0, -3.0), 0.75 -> (6.0, -3.0),
        lerp_negative_coords_half: (-8.0, -2.0), (4.0, -10.0), 0.5 -> (-2.0, -6.0),
        lerp_mixed_sign_quarter: (-2.0, 5.0), (6.0, -3.0), 0.25 -> (0.0, 3.0),
        lerp_large_symmetric_half: (1000000.0, -1000000.0), (-1000000.0, 1000000.0), 0.5 -> (0.0, 0.0),
        lerp_binary_fractions_half: (0.125, 0.5), (1.125, -0.5), 0.5 -> (0.625, 0.0),
    ];
}
