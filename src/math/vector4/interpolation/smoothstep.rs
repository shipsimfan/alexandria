use crate::math::{
    Vector4,
    number::{Clamp, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Div, Mul, Sub},
};

impl<T> Vector4<T> {
    /// Interpolates using the smoothstep algorithm between two vectors
    pub const fn smoothstep(self, other: Self, t: T) -> Self
    where
        T: [const] Sub<Output = T>
            + [const] Sub<Vector4<T>, Output = Vector4<T>>
            + [const] Mul<Output = T>
            + [const] Mul<Vector4<T>, Output = Vector4<T>>
            + [const] Div<Output = T>
            + [const] Clamp
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
        T::Bound: [const] Clone + [const] Destruct + Zero + One,
    {
        let x = ((t - self.clone()) / (other - self)).clamp(T::Bound::ZERO, T::Bound::ONE);
        x.clone() * x.clone() * (T::from_f32(3.0) - T::from_f32(2.0) * x)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector4f;

    macro_rules! smoothstep_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal, $i1z: literal, $i1w: literal),
                ($i2x: literal, $i2y: literal, $i2z: literal, $i2w: literal),
                $t: literal
                -> ($ox: literal, $oy: literal, $oz: literal, $ow: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector4f = Vector4f::new($i1x, $i1y, $i1z, $i1w);
                const INPUT2: Vector4f = Vector4f::new($i2x, $i2y, $i2z, $i2w);
                const OUTPUT: Vector4f = Vector4f::new($ox, $oy, $oz, $ow);
                const T: f32 = $t;

                let output = INPUT1.smoothstep(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "smoothstep failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    smoothstep_tests![
        smoothstep_below_all: (-0.0, -0.0, -0.0, -0.0), (1.0, 1.0, 1.0, 1.0), -0.5 -> (0.0, 0.0, 0.0, 0.0),
        smoothstep_at_edge0: (0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 1.0, 1.0), 0.0 -> (0.0, 0.0, 0.0, 0.0),
        smoothstep_quarter_unit: (0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 1.0, 1.0), 0.25 -> (0.15625, 0.15625, 0.15625, 0.15625),
        smoothstep_midpoint_unit: (0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 1.0, 1.0), 0.5 -> (0.5, 0.5, 0.5, 0.5),
        smoothstep_at_edge1: (0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 1.0, 1.0), 1.0 -> (1.0, 1.0, 1.0, 1.0),
        smoothstep_above_all: (0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 1.0, 1.0), 1.5 -> (1.0, 1.0, 1.0, 1.0),

        smoothstep_mixed_edges: (-1.0, 0.0, 2.0, -4.0), (1.0, 2.0, 6.0, 0.0), 1.0 -> (1.0, 0.5, 0.0, 1.0),
        smoothstep_different_ranges: (0.0, 10.0, -2.0, 3.0), (2.0, 20.0, 2.0, 7.0), 5.0 -> (1.0, 0.0, 1.0, 0.5),
        smoothstep_negative_t_inside: (-5.0, -3.0, -1.0, 1.0), (-1.0, 1.0, 3.0, 5.0), -2.0 -> (0.84375, 0.15625, 0.0, 0.0),

        smoothstep_tight_range: (0.0, 0.0, 0.0, 0.0), (0.1, 0.2, 0.4, 0.8), 0.05 -> (0.5, 0.15625, 0.04296875, 0.01123046875),
        smoothstep_partial_clamp: (0.0, 0.0, 0.0, 0.0), (1.0, 2.0, 3.0, 4.0), 1.5 -> (1.0, 0.84375, 0.5, 0.31640625),
    ];
}
