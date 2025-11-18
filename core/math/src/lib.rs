pub mod mat4;
pub mod quat;
pub mod texture;
pub mod vec3;
use crate::vec3::*;

pub struct Triangle2 {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

pub struct Triangle3 {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

#[repr(C)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn a(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn b(&self) -> Vec2 {
        Vec2 {
            x: self.x + self.w,
            y: self.y,
        }
    }

    pub fn c(&self) -> Vec2 {
        Vec2 {
            x: self.x + self.w,
            y: self.y + self.h,
        }
    }

    pub fn d(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y + self.h,
        }
    }

    pub fn tri1(&self) -> Triangle2 {
        Triangle2 {
            a: self.a(),
            b: self.b(),
            c: self.c(),
        }
    }

    pub fn tri2(&self) -> Triangle2 {
        Triangle2 {
            a: self.a(),
            b: self.c(),
            c: self.d(),
        }
    }
}

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
