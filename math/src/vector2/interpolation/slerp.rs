use crate::{
    Vector2,
    number::{Abs, Atan2, Cos, FromF32, One, Sin, Sqrt, Zero},
};
use std::ops::{Add, Div, DivAssign, Mul, Neg, Rem, Sub};

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
        + Rem<Output = T>
        + DivAssign
        + Atan2
        + Sin
        + Cos
        + Sqrt
        + Abs
        + Clone
        + PartialOrd
        + Zero
        + One
        + FromF32,
> Vector2<T>
{
    /// Interpolates spherically between two vectors
    pub fn slerp_unclamped(self, other: Self, t: T) -> Self {
        let dot = self.clone().dot(other.clone());
        if dot < T::from_f32(-1.0 + 1e-5) {
            let perp = self.clone().perp_cw();

            let angle = T::from_f32(std::f32::consts::PI) * t;
            return self * angle.clone().cos() + perp * angle.sin();
        }

        let angle = -self.clone().cross(other.clone()).atan2(dot);
        let angle_sin = angle.clone().sin();

        let a = ((T::ONE - t.clone()) * angle.clone()).sin() / angle_sin.clone();
        let b = (t * angle).sin() / angle_sin;

        self.zip3(other, Vector2::splat(a), Vector2::splat(b), slerp_unclamped)
    }
}

impl<T> Vector2<T> {
    /// Interpolates spherically between two vectors, clamping `t` between 0 and 1
    pub fn slerp(self, other: Self, t: T) -> Self
    where
        T: Mul<Output = T>
            + Sub<Output = T>
            + Add<Output = T>
            + Div<Output = T>
            + Neg<Output = T>
            + Rem<Output = T>
            + DivAssign
            + Atan2
            + Sin
            + Cos
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
    use crate::Vector2f;

    macro_rules! slerp_tests {
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

                let output = INPUT1.slerp_unclamped(INPUT2, T);

                assert!(output.approx_eq(OUTPUT, 1e-6), "slerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    slerp_tests![
        slerp_t0_returns_first: (1.0, 0.0), (0.0, 1.0), 0.0 -> (1.0, 0.0),
        slerp_t1_returns_second: (1.0, 0.0), (0.0, 1.0), 1.0 -> (0.0, 1.0),

        slerp_quarter_90deg: (1.0, 0.0), (0.0, 1.0), 0.25 -> (0.92387953, 0.38268343),
        slerp_half_90deg: (1.0, 0.0), (0.0, 1.0), 0.5 -> (0.70710677, 0.70710677),
        slerp_three_quarter_90deg: (1.0, 0.0), (0.0, 1.0), 0.75 -> (0.38268343, 0.92387953),

        slerp_half_up_to_left: (0.0, 1.0), (-1.0, 0.0), 0.5 -> (-0.70710677, 0.70710677),
        slerp_half_right_to_down: (1.0, 0.0), (0.0, -1.0), 0.5 -> (0.70710677, -0.70710677),

        slerp_opposite_half_left_hand: (1.0, 0.0), (-1.0, 0.0), 0.5 -> (0.0, -1.0),
        slerp_opposite_quarter_left_hand: (1.0, 0.0), (-1.0, 0.0), 0.25 -> (0.70710677, -0.70710677),
        slerp_opposite_three_quarter_left_hand: (1.0, 0.0), (-1.0, 0.0), 0.75 -> (-0.70710677, -0.70710677),
    ];
}
