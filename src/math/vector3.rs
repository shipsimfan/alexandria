use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::Vector2;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn cross(&self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normal(self) -> Vector3 {
        self / self.magnitude()
    }

    pub fn normalize(&mut self) {
        *self = self.normal()
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn xx(&self) -> Vector2 {
        Vector2::new(self.x, self.x)
    }

    pub fn xy(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    pub fn xz(&self) -> Vector2 {
        Vector2::new(self.x, self.z)
    }

    pub fn yx(&self) -> Vector2 {
        Vector2::new(self.y, self.x)
    }

    pub fn yy(&self) -> Vector2 {
        Vector2::new(self.y, self.y)
    }

    pub fn yz(&self) -> Vector2 {
        Vector2::new(self.y, self.z)
    }

    pub fn zx(&self) -> Vector2 {
        Vector2::new(self.z, self.x)
    }

    pub fn zy(&self) -> Vector2 {
        Vector2::new(self.z, self.y)
    }

    pub fn zz(&self) -> Vector2 {
        Vector2::new(self.z, self.z)
    }

    pub fn xxx(&self) -> Vector3 {
        Vector3::new(self.x, self.x, self.x)
    }

    pub fn xxy(&self) -> Vector3 {
        Vector3::new(self.x, self.x, self.y)
    }

    pub fn xxz(&self) -> Vector3 {
        Vector3::new(self.x, self.x, self.z)
    }

    pub fn xyx(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.x)
    }

    pub fn xyy(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.y)
    }

    pub fn xzx(&self) -> Vector3 {
        Vector3::new(self.x, self.z, self.x)
    }

    pub fn xzy(&self) -> Vector3 {
        Vector3::new(self.x, self.z, self.y)
    }

    pub fn xzz(&self) -> Vector3 {
        Vector3::new(self.x, self.z, self.z)
    }

    pub fn yxx(&self) -> Vector3 {
        Vector3::new(self.y, self.x, self.x)
    }

    pub fn yxy(&self) -> Vector3 {
        Vector3::new(self.y, self.x, self.y)
    }

    pub fn yxz(&self) -> Vector3 {
        Vector3::new(self.y, self.x, self.z)
    }

    pub fn yyx(&self) -> Vector3 {
        Vector3::new(self.y, self.y, self.x)
    }

    pub fn yyy(&self) -> Vector3 {
        Vector3::new(self.y, self.y, self.y)
    }

    pub fn yyz(&self) -> Vector3 {
        Vector3::new(self.y, self.y, self.z)
    }

    pub fn yzx(&self) -> Vector3 {
        Vector3::new(self.y, self.z, self.x)
    }

    pub fn yzy(&self) -> Vector3 {
        Vector3::new(self.y, self.z, self.y)
    }

    pub fn yzz(&self) -> Vector3 {
        Vector3::new(self.y, self.z, self.z)
    }

    pub fn zxx(&self) -> Vector3 {
        Vector3::new(self.z, self.x, self.x)
    }

    pub fn zxy(&self) -> Vector3 {
        Vector3::new(self.z, self.x, self.y)
    }

    pub fn zxz(&self) -> Vector3 {
        Vector3::new(self.z, self.x, self.z)
    }

    pub fn zyx(&self) -> Vector3 {
        Vector3::new(self.z, self.y, self.x)
    }

    pub fn zyy(&self) -> Vector3 {
        Vector3::new(self.z, self.y, self.y)
    }

    pub fn zyz(&self) -> Vector3 {
        Vector3::new(self.z, self.y, self.z)
    }

    pub fn zzx(&self) -> Vector3 {
        Vector3::new(self.z, self.z, self.x)
    }

    pub fn zzy(&self) -> Vector3 {
        Vector3::new(self.z, self.z, self.y)
    }

    pub fn zzz(&self) -> Vector3 {
        Vector3::new(self.z, self.z, self.z)
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }
}

impl From<f32> for Vector3 {
    fn from(x: f32) -> Vector3 {
        Vector3::new(x, 0.0, 0.0)
    }
}

impl From<(f32, f32)> for Vector3 {
    fn from(val: (f32, f32)) -> Vector3 {
        Vector3::new(val.0, val.1, 0.0)
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(val: (f32, f32, f32)) -> Vector3 {
        Vector3::new(val.0, val.1, val.2)
    }
}

impl From<Vector2> for Vector3 {
    fn from(vec2: Vector2) -> Vector3 {
        Vector3::new(vec2.x(), vec2.y(), 0.0)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        *self = *self + rhs;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul for Vector3 {
    type Output = f32;

    fn mul(self, rhs: Vector3) -> Self::Output {
        self.dot(rhs)
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
