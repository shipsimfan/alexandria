use crate::{
    Matrix3x3, Vector3,
    number::{One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>
        + Sqrt
        + Clone
        + PartialEq
        + Zero
        + One,
> Matrix3x3<T>
{
    /// Create a new [`Matrix3x3`] pointing from `position` to `target`
    pub fn new_look_at(position: Vector3<T>, target: Vector3<T>, up: Vector3<T>) -> Matrix3x3<T> {
        let forward = (target - position.clone()).normalized();
        let right = up.cross(forward.clone()).normalized();
        let up = forward.clone().cross(right.clone()).normalized();

        Matrix3x3::new_rows(right, up, forward)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix3x3f, Vector3f};

    macro_rules! look_at_tests {
        [$(
            $test_name: ident:
                ([$ix: literal, $iy: literal, $iz: literal],
                 [$px: literal, $py: literal, $pz: literal],
                 [$tx: literal, $ty: literal, $tz: literal],
                 [$ux: literal, $uy: literal, $uz: literal])
                -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const POSITION: Vector3f = Vector3f::new($px, $py, $pz);
                const TARGET: Vector3f = Vector3f::new($tx, $ty, $tz);
                const UP: Vector3f = Vector3f::new($ux, $uy, $uz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let projection = Matrix3x3f::new_look_at(POSITION, TARGET, UP);

                let output = projection.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "look at failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    look_at_tests![];
}
