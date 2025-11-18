use crate::{quat::*, vec3::*};
use std::ops::Mul;

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

    pub fn perspective(fov_y: f32, aspect: f32, znear: f32, zfar: f32) -> Self {
        let f = 1.0 / (fov_y * 0.5).tan();
        let a = f / aspect;
        let b = (zfar + znear) / (znear - zfar);
        let c = (2.0 * zfar * znear) / (znear - zfar);

        Mat4 {
            m: [
                a, 0.0, 0.0, 0.0, 0.0, f, 0.0, 0.0, 0.0, 0.0, b, c, 0.0, 0.0, -1.0, 0.0,
            ],
        }
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = (target - eye).normalized(); // forward
        let s = f.cross(up).normalized(); // right
        let u = s.cross(f); // recalculated up

        Mat4 {
            m: [
                // row 0
                s.x,
                s.y,
                s.z,
                -s.dot(eye),
                // row 1
                u.x,
                u.y,
                u.z,
                -u.dot(eye),
                // row 2
                -f.x,
                -f.y,
                -f.z,
                f.dot(eye),
                // row 3
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        }
    }

    #[inline]
    pub fn translate(t: Vec3) -> Self {
        Mat4::new(
            1.0, 0.0, 0.0, t.x, 0.0, 1.0, 0.0, t.y, 0.0, 0.0, 1.0, t.z, 0.0, 0.0, 0.0, 1.0,
        )
    }

    #[inline]
    pub fn scale(s: Vec3) -> Self {
        Mat4::new(
            s.x, 0.0, 0.0, 0.0, 0.0, s.y, 0.0, 0.0, 0.0, 0.0, s.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn trs(t: Vec3, r: Quat, s: Vec3) -> Mat4 {
        Mat4::translate(t) * r.to_mat4() * Mat4::scale(s)
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
