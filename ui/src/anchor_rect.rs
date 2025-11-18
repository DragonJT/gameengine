use math::*;

pub enum Position {
    AnchorMin(f32),
    Relative(f32),
    AnchorMax(f32),
}

impl Position {
    fn calc(&self, min: f32, max: f32) -> f32 {
        match self {
            &Position::AnchorMin(p) => min + p,
            &Position::AnchorMax(p) => max - p,
            &Position::Relative(f) => min + f * (max - min),
        }
    }

    pub fn calc_horizontal(&self, rect: &Rect) -> f32 {
        self.calc(rect.x, rect.x + rect.w)
    }

    pub fn calc_vertical(&self, rect: &Rect) -> f32 {
        self.calc(rect.y, rect.y + rect.h)
    }
}

pub struct AnchorRect {
    pub left: Position,
    pub right: Position,
    pub top: Position,
    pub bottom: Position,
}

impl AnchorRect {
    pub fn calc_rect(&self, rect: &Rect) -> Rect {
        let l = self.left.calc_horizontal(rect);
        let r = self.right.calc_horizontal(rect);
        let t = self.top.calc_vertical(rect);
        let b = self.bottom.calc_vertical(rect);
        Rect {
            x: l,
            y: t,
            w: r - l,
            h: b - t,
        }
    }
}
