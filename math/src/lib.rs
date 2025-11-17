use std::ops::{Add, Mul, Neg, Sub};

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0, 1.0)
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Color {
        Color::new(0.0, 1.0, 0.0, 1.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0, 1.0)
    }
}

/// 3D vector with `f32` components.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const X: Vec3 = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const Y: Vec3 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const Z: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn normalized(self) -> Vec3 {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// Vec3 * scalar
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// scalar * Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

// Vec3 / scalar
impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

/// Unit quaternion representing rotation.
/// Stored as (w, x, y, z) with w being the scalar part.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quat {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quat {
    pub const IDENTITY: Quat = Quat {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    #[inline]
    pub const fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    /// Construct from axis-angle (axis must be non-zero; will be normalized).
    pub fn from_axis_angle(axis: Vec3, angle_rad: f32) -> Self {
        let half = angle_rad * 0.5;
        let (s, c) = half.sin_cos();
        let axis = axis.normalized();
        Quat {
            w: c,
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
        }
    }

    /// Quaternion magnitude.
    pub fn length(self) -> f32 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Return normalized (unit) quaternion.
    pub fn normalized(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            self
        } else {
            Quat {
                w: self.w / len,
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        }
    }

    /// Quaternion multiplication (rotation composition: self followed by rhs).
    pub fn mul(self, rhs: Quat) -> Quat {
        let w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
        let x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
        let y = self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x;
        let z = self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w;
        Quat { w, x, y, z }
    }

    /// Rotate a vector by this quaternion (assuming it’s unit or close).
    pub fn rotate_vec3(self, v: Vec3) -> Vec3 {
        let qv = Vec3::new(self.x, self.y, self.z);
        let t = 2.0 * qv.cross(v);
        v + self.w * t + qv.cross(t)
    }

    /// Convert quaternion to 3×3 rotation matrix part of a Mat4.
    pub fn to_mat4(self) -> Mat4 {
        let q = self.normalized();
        let (w, x, y, z) = (q.w, q.x, q.y, q.z);

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;

        Mat4::new(
            1.0 - 2.0 * (yy + zz),
            2.0 * (xy - wz),
            2.0 * (xz + wy),
            0.0,
            2.0 * (xy + wz),
            1.0 - 2.0 * (xx + zz),
            2.0 * (yz - wx),
            0.0,
            2.0 * (xz - wy),
            2.0 * (yz + wx),
            1.0 - 2.0 * (xx + yy),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }
}

// Allow `Quat * Quat` as operator.
impl Mul<Quat> for Quat {
    type Output = Quat;

    fn mul(self, rhs: Quat) -> Quat {
        self.mul(rhs)
    }
}

// Allow `Quat * Vec3` to rotate a vector.
impl Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        self.rotate_vec3(rhs)
    }
}

/// 4×4 matrix in row-major order.
///
/// m[row][col] = m[row * 4 + col]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4 {
    pub m: [f32; 16],
}

impl Mat4 {
    pub const IDENTITY: Mat4 = Mat4 {
        m: [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    };

    #[inline]
    pub const fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    ) -> Self {
        Mat4 {
            m: [
                m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33,
            ],
        }
    }

    #[inline]
    pub fn from_translation(t: Vec3) -> Self {
        Mat4::new(
            1.0, 0.0, 0.0, t.x, 0.0, 1.0, 0.0, t.y, 0.0, 0.0, 1.0, t.z, 0.0, 0.0, 0.0, 1.0,
        )
    }

    #[inline]
    pub fn from_scale(s: Vec3) -> Self {
        Mat4::new(
            s.x, 0.0, 0.0, 0.0, 0.0, s.y, 0.0, 0.0, 0.0, 0.0, s.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    /// Transform a Vec3 as a position (Vec3 augmented with w=1).
    pub fn transform_point3(self, v: Vec3) -> Vec3 {
        let x = self.m[0] * v.x + self.m[1] * v.y + self.m[2] * v.z + self.m[3];
        let y = self.m[4] * v.x + self.m[5] * v.y + self.m[6] * v.z + self.m[7];
        let z = self.m[8] * v.x + self.m[9] * v.y + self.m[10] * v.z + self.m[11];
        let w = self.m[12] * v.x + self.m[13] * v.y + self.m[14] * v.z + self.m[15];

        if w != 0.0 {
            Vec3::new(x / w, y / w, z / w)
        } else {
            Vec3::new(x, y, z)
        }
    }

    /// Transform a Vec3 as a direction (ignores translation).
    pub fn transform_dir3(self, v: Vec3) -> Vec3 {
        let x = self.m[0] * v.x + self.m[1] * v.y + self.m[2] * v.z;
        let y = self.m[4] * v.x + self.m[5] * v.y + self.m[6] * v.z;
        let z = self.m[8] * v.x + self.m[9] * v.y + self.m[10] * v.z;
        Vec3::new(x, y, z)
    }

    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let rl = right - left;
        let tb = top - bottom;
        let fn_ = far - near;

        Mat4::new(
            2.0 / rl,
            0.0,
            0.0,
            -(right + left) / rl,
            0.0,
            2.0 / tb,
            0.0,
            -(top + bottom) / tb,
            0.0,
            0.0,
            -2.0 / fn_,
            -(far + near) / fn_,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    pub fn to_f32_ptr(&self) -> *const f32 {
        self.m.as_ptr()
    }
}

// Mat4 * Mat4
impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        let mut r = [0.0f32; 16];
        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.m[row * 4 + k] * rhs.m[k * 4 + col];
                }
                r[row * 4 + col] = sum;
            }
        }
        Mat4 { m: r }
    }
}

// Mat4 * Vec3 -> position transform
impl Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        self.transform_point3(rhs)
    }
}
