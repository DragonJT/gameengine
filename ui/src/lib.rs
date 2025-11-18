pub mod anchor_rect;
pub mod widget;
use math::*;

#[derive(Debug)]
pub struct DrawRect {
    pub rect: Rect,
    pub color: Color,
    pub outline: Option<f32>,
}

#[derive(Debug)]
pub struct DrawLine {
    pub a: Vec2,
    pub b: Vec2,
    pub color: Color,
}

#[derive(Debug)]
pub struct DrawText {
    pub position: Vec2,
    pub text: String,
    pub color: Color,
    pub fontscale: f32,
}

#[derive(Debug)]
pub enum DrawType {
    DrawRect(DrawRect),
    DrawLine(DrawLine),
    DrawText(DrawText),
}
