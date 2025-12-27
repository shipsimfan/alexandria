use crate::{
    Quaternion,
    number::{Abs, Acos, Atan2, FromF32, One, Sin, Sqrt, Zero},
};
use std::ops::{Add, Div, DivAssign, Mul, Neg, Sub};

const fn slerp_unclamped<T: [const] Add<Output = T> + [const] Mul<Output = T>>(
    lhs: T,
    rhs: T,
    a: T,
    b: T,
) -> T {
    lhs * a + rhs * b
}

impl<
    T: Mul<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + DivAssign
        + Atan2
        + Acos
        + Sin
        + Sqrt
        + Abs
        + Clone
        + PartialOrd
        + Zero
        + One
        + FromF32,
> Quaternion<T>
{
    /// Interpolates spherically between two vectors
    pub fn slerp_unclamped(mut self, mut other: Self, t: T) -> Self {
        self.normalize();
        other.normalize();

        let a = self.clone();
        let b = other.clone();
        let mut dot = a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;
        if dot < T::ZERO {
            dot = -dot;
            other = -other;
        }
        if dot > T::from_f32(0.9995) {
            return self.nlerp(other, t);
        }

        let angle = dot.acos();
        let angle_sin = angle.clone().sin();

        let a = ((T::ONE - t.clone()) * angle.clone()).sin() / angle_sin.clone();
        let b = (t * angle).sin() / angle_sin;

        self.zip3(
            other,
            Quaternion::splat(a),
            Quaternion::splat(b),
            slerp_unclamped,
        )
    }
}

impl<
    T: Mul<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + DivAssign
        + Atan2
        + Acos
        + Sin
        + Sqrt
        + Abs
        + Clone
        + PartialOrd
        + Zero
        + One
        + FromF32,
> Quaternion<T>
{
    /// Interpolates spherically between two vectors, clamping `t` between 0 and 1
    pub fn slerp(self, other: Self, t: T) -> Self {
        self.slerp_unclamped(
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
}

#[cfg(test)]
mod tests {
    use crate::Quaternionf;

    macro_rules! slerp_tests {
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

                let output = INPUT1.slerp_unclamped(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "slerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    slerp_tests![
        slerp_orth_xy_t0: (1.0, 0.0, 0.0, 0.0), (0.0, 1.0, 0.0, 0.0), 0.0 -> (1.0, 0.0, 0.0, 0.0),
        slerp_orth_xy_t1: (1.0, 0.0, 0.0, 0.0), (0.0, 1.0, 0.0, 0.0), 1.0 -> (0.0, 1.0, 0.0, 0.0),
        slerp_orth_xy_t05: (1.0, 0.0, 0.0, 0.0), (0.0, 1.0, 0.0, 0.0), 0.5 -> (0.707107, 0.707107, 0.0, 0.0),
        slerp_orth_xy_t025: (1.0, 0.0, 0.0, 0.0), (0.0, 1.0, 0.0, 0.0), 0.25 -> (0.92388, 0.382683, 0.0, 0.0),

        slerp_orth_zw_t05: (0.0, 0.0, 1.0, 0.0), (0.0, 0.0, 0.0, 1.0), 0.5 -> (0.0, 0.0, 0.707107, 0.707107),

        slerp_nonnormalized_inputs_t05: (2.0, 0.0, 0.0, 0.0), (0.0, 3.0, 0.0, 0.0), 0.5 -> (0.707107, 0.707107, 0.0, 0.0),

        slerp_antipodal_shortest_t03: (1.0, 0.0, 0.0, 0.0), (-1.0, 0.0, 0.0, 0.0), 0.3 -> (1.0, 0.0, 0.0, 0.0),

        slerp_arbitrary_t037: (0.2, 0.3, 0.4, 0.5), (-0.1, 0.7, -0.2, 0.6), 0.37 -> (0.143687, 0.593906, 0.287375, 0.737594),
        slerp_arbitrary_t080: (0.2, 0.3, 0.4, 0.5), (-0.1, 0.7, -0.2, 0.6), 0.8 -> (-0.026158, 0.718854, -0.052315, 0.692696),

        slerp_near_parallel_nlerp_t05: (1.0, 0.0, 0.0, 0.0), (0.9999, 0.01, 0.0, 0.0), 0.5 -> (0.999987, 0.005, 0.0, 0.0),
        slerp_near_opposite_flip_t05: (1.0, 0.0, 0.0, 0.0), (-0.999, 0.0447, 0.0, 0.0), 0.5 -> (0.99975, -0.022356, 0.0, 0.0),
    ];
}
