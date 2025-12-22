use crate::{
    Vector2,
    number::{One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T> Vector2<T> {
    /// Interpolates using the smoothstep algorithm between two vectors
    pub const fn smoothstep(self, other: Self, t: T) -> Self
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
        let t = if t < T::ZERO {
            T::ZERO
        } else if t > T::ONE {
            T::ONE
        } else {
            t.clone() * t.clone() * ((T::ONE + T::ONE + T::ONE) - (T::ONE + T::ONE) * t)
        };
        self.lerp_unclamped(other, t)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector2f;

    macro_rules! smoothstep_tests {
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

                let output = INPUT1.smoothstep(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "smoothstep failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    smoothstep_tests![
        smoothstep_clamp_low: (1.0, -2.0), (9.0, 6.0), -0.5 -> (1.0, -2.0),
        smoothstep_clamp_high: (1.0, -2.0), (9.0, 6.0), 1.5 -> (9.0, 6.0),

        smoothstep_t0: (0.0, 0.0), (10.0, -10.0), 0.0 -> (0.0, 0.0),
        smoothstep_t1: (0.0, 0.0), (10.0, -10.0), 1.0 -> (10.0, -10.0),

        smoothstep_half: (0.0, 0.0), (10.0, -10.0), 0.5 -> (5.0, -5.0),

        smoothstep_quarter: (0.0, 0.0), (10.0, -10.0), 0.25 -> (1.5625, -1.5625),
        smoothstep_three_quarter: (0.0, 0.0), (10.0, -10.0), 0.75 -> (8.4375, -8.4375),

        smoothstep_quarter_offset: (2.0, -3.0), (-6.0, 5.0), 0.25 -> (0.75, -1.75),
        smoothstep_half_offset: (2.0, -3.0), (-6.0, 5.0), 0.5 -> (-2.0, 1.0),
        smoothstep_three_quarter_offset: (2.0, -3.0), (-6.0, 5.0), 0.75 -> (-4.75, 3.75),

        smoothstep_eighth: (-1.0, 4.0), (3.0, 12.0), 0.125 -> (-0.828125, 4.34375),
        smoothstep_seven_eighth: (-1.0, 4.0), (3.0, 12.0), 0.875 -> (2.828125, 11.65625),

        smoothstep_degenerate: (5.0, -2.0), (5.0, -2.0), 0.75 -> (5.0, -2.0),
    ];
}
