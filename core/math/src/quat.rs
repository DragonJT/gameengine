use crate::{mat4::*, vec3::*};
use std::ops::Mul;

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

    /// Create an orientation so that local +Z points along `forward`
    /// and local +Y is aligned with `up` as much as possible.
    pub fn from_forward_up(forward: Vec3, up: Vec3) -> Self {
        let f = forward.normalized();
        let r = up.cross(f).normalized(); // right
        let u = f.cross(r); // corrected up

        // Build a 3x3 rotation matrix from the basis:
        // columns = right, up, forward
        let m00 = r.x;
        let m01 = u.x;
        let m02 = f.x;
        let m10 = r.y;
        let m11 = u.y;
        let m12 = f.y;
        let m20 = r.z;
        let m21 = u.z;
        let m22 = f.z;

        // Convert rotation matrix to quaternion
        let trace = m00 + m11 + m22;

        let (w, x, y, z) = if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0; // s = 4*w
            let w = 0.25 * s;
            let x = (m21 - m12) / s;
            let y = (m02 - m20) / s;
            let z = (m10 - m01) / s;
            (w, x, y, z)
        } else if m00 > m11 && m00 > m22 {
            let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0; // s = 4*x
            let w = (m21 - m12) / s;
            let x = 0.25 * s;
            let y = (m01 + m10) / s;
            let z = (m02 + m20) / s;
            (w, x, y, z)
        } else if m11 > m22 {
            let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0; // s = 4*y
            let w = (m02 - m20) / s;
            let x = (m01 + m10) / s;
            let y = 0.25 * s;
            let z = (m12 + m21) / s;
            (w, x, y, z)
        } else {
            let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0; // s = 4*z
            let w = (m10 - m01) / s;
            let x = (m02 + m20) / s;
            let y = (m12 + m21) / s;
            let z = 0.25 * s;
            (w, x, y, z)
        };

        Quat { w, x, y, z }.normalized()
    }

    /// Quaternion that makes the object at `eye` look toward `target`.
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (target - eye).normalized();
        Self::from_forward_up(forward, up)
    }

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
