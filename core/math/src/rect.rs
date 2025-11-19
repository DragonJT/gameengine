use crate::*;
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect { x, y, w, h }
    }

    pub fn from_vec2s(pos: Vec2, size: Vec2) -> Self {
        Rect {
            x: pos.x,
            y: pos.y,
            w: size.x,
            h: size.y,
        }
    }

    pub fn expand(&self, amount: f32) -> Self {
        Rect {
            x: self.x - amount,
            y: self.y - amount,
            w: self.w + amount * 2.0,
            h: self.h + amount * 2.0,
        }
    }

    pub fn contains(&self, v: &Vec2) -> bool {
        v.x >= self.x && v.x <= self.x + self.w && v.y >= self.y && v.y <= self.y + self.h
    }

    pub fn topleft(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn size(&self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.h,
        }
    }

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

impl Add<Vec2> for Rect {
    type Output = Rect;

    #[inline]
    fn add(self, rhs: Vec2) -> Rect {
        Rect::new(self.x + rhs.x, self.y + rhs.y, self.w, self.h)
    }
}
