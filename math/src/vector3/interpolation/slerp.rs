use crate::{
    Vector3,
    number::{Abs, Atan2, FromF32, One, Sin, Sqrt, Zero},
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
        + Sin
        + Sqrt
        + Abs
        + Clone
        + PartialOrd
        + Zero
        + One
        + FromF32,
> Vector3<T>
{
    /// Interpolates spherically between two vectors
    pub fn slerp_unclamped(mut self, mut other: Self, t: T) -> Self {
        self.normalize();
        other.normalize();

        let dot = self.clone().dot(other.clone());
        if dot.abs() > T::from_f32(0.9995) {
            return self.nlerp(other, t);
        }

        let angle = self.clone().angle_between(other.clone());

        let angle_sin = angle.clone().sin();

        let a = ((T::ONE - t.clone()) * angle.clone()).sin() / angle_sin.clone();
        let b = (t * angle).sin() / angle_sin;

        self.zip3(other, Vector3::splat(a), Vector3::splat(b), slerp_unclamped)
    }
}

impl<T> Vector3<T> {
    /// Interpolates spherically between two vectors, clamping `t` between 0 and 1
    pub fn slerp(self, other: Self, t: T) -> Self
    where
        T: Mul<Output = T>
            + Sub<Output = T>
            + Add<Output = T>
            + Div<Output = T>
            + Neg<Output = T>
            + DivAssign
            + Atan2
            + Sin
            + Sqrt
            + Abs
            + Clone
            + PartialOrd
            + Zero
            + One
            + FromF32,
    {
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
    use crate::Vector3f;

    macro_rules! slerp_tests {
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

                let output = INPUT1.slerp_unclamped(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "slerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    slerp_tests![
        slerp_axis_xy_t0: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.0 -> (1.0, 0.0, 0.0),
        slerp_axis_xy_t1: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 1.0 -> (0.0, 1.0, 0.0),
        slerp_axis_xy_t05: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.5 -> (0.707106781187, 0.707106781187, 0.0),
        slerp_axis_xy_t025: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.25 -> (0.923879532511, 0.382683432365, 0.0),
        slerp_axis_xy_t075: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.75 -> (0.382683432365, 0.923879532511, 0.0),

        slerp_axis_xz_t05: (1.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5 -> (0.707106781187, 0.0, 0.707106781187),
        slerp_axis_yz_t05: (0.0, 1.0, 0.0), (0.0, 0.0, 1.0), 0.5 -> (0.0, 0.707106781187, 0.707106781187),

        slerp_angle60_xy_t05: (1.0, 0.0, 0.0), (0.5, 0.866025403784, 0.0), 0.5 -> (0.866025403784, 0.5, 0.0),
        slerp_angle60_xy_t0333333333: (1.0, 0.0, 0.0), (0.5, 0.866025403784, 0.0), 0.333333333333 -> (0.939692620786, 0.342020143326, 0.0),

        slerp_angle120_xy_t05: (1.0, 0.0, 0.0), (-0.5, 0.866025403784, 0.0), 0.5 -> (0.5, 0.866025403784, 0.0),

        slerp_same_vector_t05: (0.57735026919, 0.57735026919, 0.57735026919), (0.57735026919, 0.57735026919, 0.57735026919), 0.5 -> (0.57735026919, 0.57735026919, 0.57735026919),

        slerp_negx_to_posy_t05: (-1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.5 -> (-0.707106781187, 0.707106781187, 0.0),
    ];
}
