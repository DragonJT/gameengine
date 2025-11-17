use cengine_rust::*;
use math::*;

fn main() {
    initialize(2000, 1600);
    let mut renderer = Renderer::new("assets/JetBrainsMono-Medium.ttf", 75.0, 512);

    renderer.draw_text(100.0, 100.0, "HelloWorld", 300.0, &Color::white());

    while !window_should_close() {
        renderer.render();
    }
}
