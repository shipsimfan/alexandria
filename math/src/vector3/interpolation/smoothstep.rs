use crate::{
    Vector3,
    number::{One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T> Vector3<T> {
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
    use crate::Vector3f;

    macro_rules! smoothstep_tests {
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

                let output = INPUT1.smoothstep(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "smoothstep failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    smoothstep_tests![
        smoothstep_t_neg_clamp: (0.0, 0.0, 0.0), (1.0, 2.0, 3.0), -0.5 -> (0.0, 0.0, 0.0),
        smoothstep_t_zero: (1.5, -2.0, 0.25), (4.5, 2.0, -0.75), 0.0 -> (1.5, -2.0, 0.25),
        smoothstep_t_one: (-1.0, -1.0, -1.0), (2.0, 3.0, 4.0), 1.0 -> (2.0, 3.0, 4.0),
        smoothstep_t_over_clamp: (-3.0, 0.5, 10.0), (7.0, -1.5, -2.0), 2.0 -> (7.0, -1.5, -2.0),

        smoothstep_t_half: (0.0, 0.0, 0.0), (10.0, -10.0, 2.0), 0.5 -> (5.0, -5.0, 1.0),
        smoothstep_t_quarter: (2.0, -4.0, 6.0), (-2.0, 0.0, 10.0), 0.25 -> (1.375, -3.375, 6.625),
        smoothstep_t_three_quarters: (2.0, -4.0, 6.0), (-2.0, 0.0, 10.0), 0.75 -> (-1.375, -0.625, 9.375),

        smoothstep_t_point1: (-1.25, 3.5, -4.0), (5.75, -0.5, 2.0), 0.1 -> (-1.054, 3.388, -3.832),
        smoothstep_t_point9: (-1.25, 3.5, -4.0), (5.75, -0.5, 2.0), 0.9 -> (5.554, -0.388, 1.832),

        smoothstep_swap_edges: (5.0, 5.0, 5.0), (1.0, 2.0, 3.0), 0.3 -> (4.136, 4.352, 4.568),
        smoothstep_equal_vectors: (1.25, -2.5, 3.75), (1.25, -2.5, 3.75), 0.42 -> (1.25, -2.5, 3.75),

        smoothstep_t_third: (-2.0, 4.0, -6.0), (4.0, -2.0, 0.0), 0.33333334 -> (-0.44444445, 2.4444444, -4.4444447),
        smoothstep_t_two_thirds: (-2.0, 4.0, -6.0), (4.0, -2.0, 0.0), 0.6666667 -> (2.4444444, -0.44444445, -1.5555556),
    ];
}
