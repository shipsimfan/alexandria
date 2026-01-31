use crate::math::Vector3;
use std::{
    marker::Destruct,
    ops::{Mul, Sub},
};

impl<T> Vector3<T> {
    /// Get the cross product of two [`Vector3`]s
    pub const fn cross(self, other: Self) -> Self
    where
        T: [const] Mul<Output = T> + [const] Sub<Output = T> + [const] Clone + [const] Destruct,
    {
        Vector3::new(
            self.y.clone() * other.z.clone() - self.z.clone() * other.y.clone(),
            self.z * other.x.clone() - self.x.clone() * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector3f;

    macro_rules! cross_tests {
        [$(
            $test_name: ident:
                ($i1x: literal, $i1y: literal, $i1z: literal),
                ($i2x: literal, $i2y: literal, $i2z: literal)
                -> ($ox: literal, $oy: literal, $oz: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Vector3f = Vector3f::new($i1x, $i1y, $i1z);
                const INPUT2: Vector3f = Vector3f::new($i2x, $i2y, $i2z);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let output = INPUT1.cross(INPUT2);

                assert!(output.approx_eq(OUTPUT, 1e-6), "cross failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    cross_tests![
        cross_e1xe2: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0) -> (0.0, 0.0, 1.0),
        cross_e2xe1: (0.0, 1.0, 0.0), (1.0, 0.0, 0.0) -> (0.0, 0.0, -1.0),
        cross_e1xe3: (1.0, 0.0, 0.0), (0.0, 0.0, 1.0) -> (0.0, -1.0, 0.0),
        cross_e3xe1: (0.0, 0.0, 1.0), (1.0, 0.0, 0.0) -> (0.0, 1.0, 0.0),

        cross_parallel_zero: (2.0, 4.0, 6.0), (1.0, 2.0, 3.0) -> (0.0, 0.0, 0.0),
        cross_antiparallel_zero: (1.0, 2.0, 3.0), (-1.0, -2.0, -3.0) -> (0.0, 0.0, 0.0),
        cross_with_zero_vector: (5.0, -4.0, 2.0), (0.0, 0.0, 0.0) -> (0.0, 0.0, 0.0),

        cross_general_a: (3.0, -2.0, 5.0), (4.0, 1.0, -1.0) -> (-3.0, 23.0, 11.0),
        cross_general_a_swapped: (4.0, 1.0, -1.0), (3.0, -2.0, 5.0) -> (3.0, -23.0, -11.0),

        cross_mixed_zeros: (0.0, 7.0, -3.0), (2.0, 0.0, 5.0) -> (35.0, -6.0, -14.0),
        cross_axis_offset: (-2.0, 0.0, 1.0), (0.0, -3.0, 4.0) -> (3.0, 8.0, 6.0),
        cross_all_negative_vs_mixed: (-1.0, -1.0, -1.0), (2.0, -3.0, 4.0) -> (-7.0, 2.0, 5.0),
    ];
}
