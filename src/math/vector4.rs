use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{Vector2, Vector3};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 { x, y, z, w }
    }

    pub fn dot(&self, rhs: Vector4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
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

    pub fn w(&self) -> f32 {
        self.w
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

    pub fn xw(&self) -> Vector2 {
        Vector2::new(self.x, self.w)
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

    pub fn yw(&self) -> Vector2 {
        Vector2::new(self.y, self.w)
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

    pub fn zw(&self) -> Vector2 {
        Vector2::new(self.z, self.w)
    }

    pub fn wx(&self) -> Vector2 {
        Vector2::new(self.w, self.x)
    }

    pub fn wy(&self) -> Vector2 {
        Vector2::new(self.w, self.y)
    }

    pub fn wz(&self) -> Vector2 {
        Vector2::new(self.w, self.z)
    }

    pub fn ww(&self) -> Vector2 {
        Vector2::new(self.w, self.w)
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

    pub fn xxw(&self) -> Vector3 {
        Vector3::new(self.x, self.x, self.w)
    }

    pub fn xyx(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.x)
    }

    pub fn xyy(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.y)
    }

    pub fn xyw(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.w)
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

    pub fn xzw(&self) -> Vector3 {
        Vector3::new(self.x, self.z, self.w)
    }

    pub fn xwx(&self) -> Vector3 {
        Vector3::new(self.x, self.w, self.x)
    }

    pub fn xwy(&self) -> Vector3 {
        Vector3::new(self.x, self.w, self.y)
    }

    pub fn xwz(&self) -> Vector3 {
        Vector3::new(self.x, self.w, self.z)
    }

    pub fn xww(&self) -> Vector3 {
        Vector3::new(self.x, self.w, self.w)
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

    pub fn yxw(&self) -> Vector3 {
        Vector3::new(self.y, self.x, self.w)
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

    pub fn yyw(&self) -> Vector3 {
        Vector3::new(self.y, self.y, self.w)
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

    pub fn yzw(&self) -> Vector3 {
        Vector3::new(self.y, self.z, self.w)
    }

    pub fn ywx(&self) -> Vector3 {
        Vector3::new(self.y, self.w, self.x)
    }

    pub fn ywy(&self) -> Vector3 {
        Vector3::new(self.y, self.w, self.y)
    }

    pub fn ywz(&self) -> Vector3 {
        Vector3::new(self.y, self.w, self.z)
    }

    pub fn yww(&self) -> Vector3 {
        Vector3::new(self.y, self.w, self.w)
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

    pub fn zxw(&self) -> Vector3 {
        Vector3::new(self.z, self.x, self.w)
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

    pub fn zyw(&self) -> Vector3 {
        Vector3::new(self.z, self.y, self.w)
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

    pub fn zzw(&self) -> Vector3 {
        Vector3::new(self.z, self.z, self.w)
    }

    pub fn zwx(&self) -> Vector3 {
        Vector3::new(self.z, self.w, self.x)
    }

    pub fn zwy(&self) -> Vector3 {
        Vector3::new(self.z, self.w, self.y)
    }

    pub fn zwz(&self) -> Vector3 {
        Vector3::new(self.z, self.w, self.z)
    }

    pub fn zww(&self) -> Vector3 {
        Vector3::new(self.z, self.w, self.w)
    }

    pub fn xxxx(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.x, self.x)
    }

    pub fn xxxy(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.x, self.y)
    }

    pub fn xxxz(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.x, self.z)
    }

    pub fn xxxw(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.x, self.w)
    }

    pub fn xxyx(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.y, self.x)
    }

    pub fn xxyy(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.y, self.y)
    }

    pub fn xxyz(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.y, self.z)
    }

    pub fn xxyw(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.y, self.w)
    }

    pub fn xxzx(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.z, self.x)
    }

    pub fn xxzy(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.z, self.y)
    }

    pub fn xxzz(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.z, self.z)
    }

    pub fn xxzw(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.z, self.w)
    }

    pub fn xxwx(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.w, self.x)
    }

    pub fn xxwy(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.w, self.y)
    }

    pub fn xxwz(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.w, self.z)
    }

    pub fn xxww(&self) -> Vector4 {
        Vector4::new(self.x, self.x, self.w, self.w)
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

    pub fn set_w(&mut self, w: f32) {
        self.w = w;
    }
}

impl From<f32> for Vector4 {
    fn from(x: f32) -> Vector4 {
        Vector4::new(x, 0.0, 0.0, 0.0)
    }
}

impl From<(f32, f32)> for Vector4 {
    fn from(val: (f32, f32)) -> Vector4 {
        Vector4::new(val.0, val.1, 0.0, 0.0)
    }
}

impl From<(f32, f32, f32)> for Vector4 {
    fn from(val: (f32, f32, f32)) -> Vector4 {
        Vector4::new(val.0, val.1, val.2, 0.0)
    }
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from(val: (f32, f32, f32, f32)) -> Vector4 {
        Vector4::new(val.0, val.1, val.2, val.3)
    }
}

impl From<Vector2> for Vector4 {
    fn from(vec2: Vector2) -> Vector4 {
        Vector4::new(vec2.x(), vec2.y(), 0.0, 0.0)
    }
}

impl From<Vector3> for Vector4 {
    fn from(vec3: Vector3) -> Vector4 {
        Vector4::new(vec3.x(), vec3.y(), vec3.z(), 0.0)
    }
}

impl Add for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Self::Output {
        Vector4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Vector4) {
        *self = *self + rhs;
    }
}

impl Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Vector4 {
        Vector4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl SubAssign for Vector4 {
    fn sub_assign(&mut self, rhs: Vector4) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Mul for Vector4 {
    type Output = f32;

    fn mul(self, rhs: Vector4) -> Self::Output {
        self.dot(rhs)
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: f32) -> Self::Output {
        Vector4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
