use cengine_rust::{text_renderer::*, *};
use math::*;

fn main() {
    initialize(2000, 1600);
    let mut text_renderer = TextRenderer::new("assets/JetBrainsMono-Medium.ttf", 75.0, 512);

    while !window_should_close() {
        text_renderer.draw_rect(
            &Rect {
                x: 100.0,
                y: 100.0,
                w: 500.0,
                h: 500.0,
            },
            &Color::red(),
        );
        text_renderer.draw_text(
            &Vec2 { x: 100.0, y: 100.0 },
            "HelloWorld",
            300.0,
            &Color::white(),
        );
        text_renderer.render();
    }
}
