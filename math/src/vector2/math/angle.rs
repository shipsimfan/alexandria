use crate::{Vector2, number::Atan2};
use std::ops::Neg;

impl<T: Neg<Output = T> + Atan2> Vector2<T> {
    /// Calculate the clockwise angle from the x-axis
    pub fn angle(self) -> T {
        -self.y.atan2(self.x)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector2f, number::ApproxEq};

    macro_rules! angle_tests {
        [$(
            $test_name: ident: ($ix: literal, $iy: literal) -> $o: expr,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector2f = Vector2f::new($ix, $iy);
                const OUTPUT: f32 = $o;

                let output = INPUT.angle();

                assert!(output.approx_eq(OUTPUT, 1e-6), "angle failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    angle_tests![
        angle_pos_x: (1.0, 0.0)-> 0.0,
        angle_pos_x_scaled: (5.0, 0.0)-> 0.0,

        angle_pos_y: (0.0, 1.0)-> -std::f32::consts::FRAC_PI_2,
        angle_neg_y: (0.0, -1.0)-> std::f32::consts::FRAC_PI_2,

        angle_neg_x: (-1.0, 0.0)-> -std::f32::consts::PI,

        angle_q4_45deg: (1.0, -1.0)-> std::f32::consts::FRAC_PI_4,
        angle_q4_45deg_scaled: (10.0, -10.0)-> std::f32::consts::FRAC_PI_4,

        angle_q1_minus45deg: (1.0, 1.0)-> -std::f32::consts::FRAC_PI_4,
        angle_q1_minus45deg_scaled: (8.0, 8.0)-> -std::f32::consts::FRAC_PI_4,

        angle_q2_minus135deg: (-1.0, 1.0)-> -(3.0 * std::f32::consts::FRAC_PI_4),
        angle_q3_plus135deg: (-1.0, -1.0)-> 3.0 * std::f32::consts::FRAC_PI_4,
    ];
}
