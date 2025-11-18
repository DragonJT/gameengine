use crate::{anchor_rect::*, *};

struct Text {
    text: String,
    fontscale: f32,
    color: Color,
}

struct Outline {
    color: Color,
    width: f32,
}

struct Input {
    other: Vec<usize>,
    color: Color,
    radius: f32,
}

enum Connection {
    None,
    Input(Input),
    Output,
}

pub struct Widget {
    id: usize,
    rect: AnchorRect,
    color: Option<Color>,
    text: Option<Text>,
    outline: Option<Outline>,
    connection: Option<Connection>,
    children: Vec<Widget>,
}

impl Widget {
    pub fn screen(color: Color) -> Widget {
        Widget {
            id: 0,
            rect: AnchorRect {
                left: Position::AnchorMin(0.0),
                right: Position::AnchorMax(0.0),
                top: Position::AnchorMin(0.0),
                bottom: Position::AnchorMax(0.0),
            },
            color: Some(color),
            text: None,
            outline: None,
            connection: None,
            children: vec![],
        }
    }

    fn panel(&mut self, text: Option<Text>, rect: Rect) {
        let panel_bg_color = Color::blue();
        let panel_outline_color = Color::black();

        self.children.push(Widget {
            id: self.children.len(),
            rect: AnchorRect {
                left: Position::AnchorMin(rect.x),
                right: Position::AnchorMin(rect.x + rect.w),
                top: Position::AnchorMin(rect.y),
                bottom: Position::AnchorMin(rect.y + rect.h),
            },
            color: Some(panel_bg_color),
            text,
            outline: Some(Outline {
                color: panel_outline_color,
                width: 5.0,
            }),
            connection: None,
            children: vec![],
        });
    }

    pub fn simple_panel(&mut self, rect: Rect) {
        self.panel(None, rect);
    }

    pub fn text_panel(&mut self, text: String, rect: Rect) {
        let panel_text_color = Color::black();
        let wtext = Some(Text {
            text,
            fontscale: 1.25,
            color: panel_text_color,
        });
        self.panel(wtext, rect);
    }

    fn draw_inner(&self, drawables: &mut Vec<DrawType>, rect: &Rect) {
        let rect = &self.rect.calc_rect(rect);
        match self.color {
            Some(color) => drawables.push(DrawType::DrawRect(DrawRect {
                rect: rect.clone(),
                color,
                outline: None,
            })),
            None => {}
        }
        match &self.outline {
            Some(outline) => drawables.push(DrawType::DrawRect(DrawRect {
                rect: rect.clone(),
                color: outline.color,
                outline: Some(outline.width),
            })),
            None => {}
        }
        match &self.text {
            Some(text) => drawables.push(DrawType::DrawText(DrawText {
                position: rect.topleft(),
                text: text.text.clone(),
                color: text.color,
                fontscale: text.fontscale,
            })),
            None => {}
        }
        for c in &self.children {
            c.draw_inner(drawables, rect);
        }
    }

    pub fn draw(&self, rect: &Rect) -> Vec<DrawType> {
        let mut drawables: Vec<DrawType> = vec![];
        self.draw_inner(&mut drawables, rect);
        drawables
    }
}
