use crate::math::{
    Vector3,
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
> Vector3<T>
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
    use crate::math::Vector3f;

    macro_rules! nlerp_tests {
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

                let output = INPUT1.nlerp_unclamped(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "nlerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    nlerp_tests![
        nlerp_t0_same_as_norm_a: (2.0, 0.0, 0.0), (0.0, 5.0, 0.0), 0.0 -> (1.0, 0.0, 0.0),
        nlerp_t1_same_as_norm_b: (2.0, 0.0, 0.0), (0.0, 5.0, 0.0), 1.0 -> (0.0, 1.0, 0.0),
        nlerp_mid_orthogonal: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.5 -> (0.707106781, 0.707106781, 0.0),
        nlerp_mid_opposite_but_not_zero: (1.0, 0.0, 0.0), (-0.5, 1.0, 0.0), 0.5 -> (0.447213595, 0.894427191, 0.0),
        nlerp_skew_general: (1.0, 2.0, 3.0), (4.0, 5.0, 6.0), 0.25 -> (0.352208224, 0.553470066, 0.754731908),
        nlerp_skew_general_t075: (1.0, 2.0, 3.0), (4.0, 5.0, 6.0), 0.75 -> (0.433574275, 0.566981744, 0.700389213),
        nlerp_negative_t_extrap: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), -0.5 -> (0.948683298, -0.316227766, 0.0),
        nlerp_above1_t_extrap: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 1.5 -> (-0.316227766, 0.948683298, 0.0),
        nlerp_near_zero_component: (0.001, 2.0, 0.0), (3.0, 0.004, 0.0), 0.6 -> (0.913392649, 0.407079683, 0.0),
        nlerp_3d_nontrivial: (-2.0, 1.0, 4.0), (5.0, -3.0, 0.5), 0.33 -> (0.107651057, -0.111123672, 0.987958896),
    ];
}
