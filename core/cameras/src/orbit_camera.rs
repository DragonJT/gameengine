use math::{mat4::*, vec3::*};

pub struct OrbitCamera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,

    pub fov_y: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl OrbitCamera {
    pub fn new(target: Vec3, distance: f32, fov_y: f32, znear: f32, zfar: f32) -> Self {
        OrbitCamera {
            target,
            distance,
            yaw: 0.0,
            pitch: 0.0,
            fov_y,
            znear,
            zfar,
        }
    }

    /// Position of the camera in world space.
    pub fn position(&self) -> Vec3 {
        // Right-handed, camera looks toward target.
        // yaw = 0 -> camera on +Z axis looking toward origin.
        let cos_pitch = self.pitch.cos();
        let sin_pitch = self.pitch.sin();
        let cos_yaw = self.yaw.cos();
        let sin_yaw = self.yaw.sin();

        let x = self.target.x + self.distance * sin_yaw * cos_pitch;
        let y = self.target.y + self.distance * sin_pitch;
        let z = self.target.z + self.distance * cos_yaw * cos_pitch;

        Vec3 { x, y, z }
    }

    pub fn view_matrix(&self) -> Mat4 {
        let eye = self.position();
        let up = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        Mat4::look_at(eye, self.target, up)
    }

    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        Mat4::perspective(self.fov_y, aspect, self.znear, self.zfar)
    }

    /// Rotate camera around target. Use mouse delta in radians * some sensitivity.
    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;

        // Clamp pitch to avoid flipping over the top/bottom.
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
        let min_pitch = -max_pitch;
        if self.pitch > max_pitch {
            self.pitch = max_pitch;
        } else if self.pitch < min_pitch {
            self.pitch = min_pitch;
        }
    }

    /// Zoom in/out (positive to zoom out if you like, tweak sign as needed).
    pub fn zoom(&mut self, delta: f32) {
        self.distance += delta;
    }

    /// Move the target in world space (pans the orbit center).
    pub fn pan(&mut self, delta: Vec3) {
        self.target.x += delta.x;
        self.target.y += delta.y;
        self.target.z += delta.z;
    }
}
