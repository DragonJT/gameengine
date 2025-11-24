pub mod mat4;
pub mod quat;
pub mod rect;
pub mod texture;
pub mod vec3;
use crate::{mat4::Mat4, vec3::*};
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Triangle2 {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}
impl Triangle2 {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Triangle2 {
        Triangle2 { a, b, c }
    }

    pub fn mul(&self, mat: Mat4) -> Triangle2 {
        let a = mat * self.a;
        let b = mat * self.b;
        let c = mat * self.c;
        Triangle2 { a, b, c }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle3 {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle3 {
    pub fn normal(&self) -> Vec3 {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        Vec3::normalized(Vec3::cross(edge1, edge2))
    }
}

#[repr(C)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0, 1.0)
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

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.r,
            y: self.g,
            z: self.b,
        }
    }
}
