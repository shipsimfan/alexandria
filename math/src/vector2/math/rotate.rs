use crate::{
    Vector2,
    number::{Cos, Sin},
};
use std::ops::{Add, Mul, Neg, Sub};

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> + Sin + Cos + Clone>
    Vector2<T>
{
    /// Rotate this vector by an arbitrary angle clockwise
    pub fn rotate(self, angle: T) -> Self {
        let angle = -angle;
        let sin_angle = angle.clone().sin();
        let cos_angle = angle.cos();
        Vector2::new(
            cos_angle.clone() * self.x.clone() - sin_angle.clone() * self.y.clone(),
            sin_angle * self.x + cos_angle * self.y,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector2f;

    macro_rules! rotate_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal), $angle: literal -> ($ox: literal, $oy: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector2f = Vector2f::new($ix, $iy);
                const OUTPUT: Vector2f = Vector2f::new($ox, $oy);
                const ANGLE: f32 = $angle;

                let output = INPUT.rotate(ANGLE);

                assert!(output.approx_eq(OUTPUT, 1e-6), "rotate failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    rotate_tests![
        rotate_identity: (1.25, -3.5), 0.0 -> (1.25, -3.5),

        rotate_cw_90_x: (1.0, 0.0), 1.57079633 -> (0.0, -1.0),
        rotate_cw_90_y: (0.0, 1.0), 1.57079633 -> (1.0, 0.0),

        rotate_cw_180: (2.0, -5.0), 3.14159265 -> (-2.0, 5.0),
        rotate_cw_270: (1.0, 2.0), 4.71238898 -> (-2.0, 1.0),
        rotate_cw_360: (-3.0, 4.0), 6.28318531 -> (-3.0, 4.0),

        rotate_cw_45_unitx: (1.0, 0.0), 0.78539816 -> (0.70710678, -0.70710678),
        rotate_ccw_45_unitx: (1.0, 0.0), -0.78539816 -> (0.70710678, 0.70710678),

        rotate_cw_30: (2.0, 0.0), 0.52359878 -> (1.73205081, -1.0),
        rotate_cw_minus_60: (0.0, 2.0), -1.04719755 -> (-1.73205081, 1.0),

        rotate_cw_45_vec: (1.0, 2.0), 0.78539816 -> (2.12132034, 0.70710678),
        rotate_cw_135_vec: (1.0, 2.0), 2.35619449 -> (0.70710678, -2.12132034),

        rotate_arbitrary_small: (3.0, -4.0), 0.1 -> (2.58567883, -4.27951691),
        rotate_arbitrary_neg: (-1.5, 2.5), -0.75 -> (-2.80163020, 0.80676403),
    ];
}
