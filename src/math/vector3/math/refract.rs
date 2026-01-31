use crate::math::{
    Vector3,
    number::{One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul, Sub};

impl<
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Sqrt + PartialOrd + Clone + One + Zero,
> Vector3<T>
where
    Vector3<T>: Mul<T, Output = Vector3<T>>,
{
    /// Refract this vector around `normal` using `eta` as ratio of indices of refraction. Assumes
    /// `normal` is a unit vector
    pub fn refract_unit(self, normal: Self, eta: T) -> Self {
        let dot = self.clone().dot(normal.clone());
        let k = T::ONE - eta.clone() * eta.clone() * (T::ONE - dot.clone() * dot.clone());

        if k < T::ZERO {
            Vector3::ZERO
        } else {
            self * eta.clone() - normal * (eta * dot + k.sqrt())
        }
    }
}

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sqrt
        + PartialOrd
        + Clone
        + One
        + Zero,
> Vector3<T>
where
    Vector3<T>: Mul<T, Output = Vector3<T>>,
{
    /// Refract this vector around `normal` using `eta` as ratio of indices of refraction
    pub fn refract(self, normal: Self, eta: T) -> Self {
        self.refract_unit(normal.normalized(), eta)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector3f;

    macro_rules! refract_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal, $iz: literal),
                ($nx: literal, $ny: literal, $nz: literal),
                $eta: literal
                -> ($ox: literal, $oy: literal, $oz: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const NORMAL: Vector3f = Vector3f::new($nx, $ny, $nz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);
                const ETA: f32 = $eta;

                let output = INPUT.refract(NORMAL, ETA);

                assert!(output.approx_eq(OUTPUT, 1e-6), "refract failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    refract_tests![
        refract_normal_enter_air_to_glass: (0.00000000, -1.00000000, 0.00000000), (0.00000000, 1.00000000, 0.00000000), 0.66666667 -> (0.00000000, -1.00000000, 0.00000000),
        refract_diag45_air_to_glass: (0.70710678, -0.70710678, 0.00000000), (0.00000000, 1.00000000, 0.00000000), 0.66666667 -> (0.47140452, -0.88191710, 0.00000000),
        refract_diag45_air_to_water: (0.00000000, -0.70710678, -0.70710678), (0.00000000, 1.00000000, 0.00000000), 0.75187970 -> (0.00000000, -0.84695836, -0.53165923),
        refract_tilted_normal_air_to_glass: (0.00000000, -1.00000000, 0.00000000), (0.00000000, 0.70710678, 0.70710678), 0.66666667 -> (0.00000000, -0.95694290, -0.29027623),
        refract_skew_z_air_to_glass: (0.19518001, 0.09759001, -0.97590007), (0.00000000, 0.00000000, 1.00000000), 0.66666667 -> (0.13012001, 0.06506000, -0.98936140),
        refract_shallow_eta_gt1: (0.10049871, -0.99493719, 0.00000000), (0.00000000, 1.00000000, 0.00000000), 1.20000000 -> (0.12059845, -0.99270137, 0.00000000),
        refract_arbitrary_vectors: (0.30000000, -0.40000000, -0.86602540), (0.20000000, 0.90000001, 0.38729831), 0.80000000 -> (0.18438912, -0.57024896, -0.80051032),
    ];
}
