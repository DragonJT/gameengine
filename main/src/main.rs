use cengine_rust::{text_renderer::*, texture_renderer::TextureRenderer, *};
use math::{texture::*, *};

fn main() {
    initialize(2000, 1600);
    let mut text_renderer = TextRenderer::new("assets/JetBrainsMono-Medium.ttf", 75.0, 512);
    let mut texture_renderer = TextureRenderer::new();
    let mut texture = Texture::new(100, 100, 4);
    texture.draw_circle(50, 50, 50, &[0, 0, 255, 255]);

    texture_renderer.update_texture(&texture);
    while !window_should_close() {
        let window_size = get_window_size();
        poll_events();
        viewport(0, 0, window_size.x, window_size.y);
        clear_color_buffer_bit(1.0, 1.0, 1.0, 1.0);
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

        texture_renderer.draw_full_texture(&Rect {
            x: 400.0,
            y: 400.0,
            w: 400.0,
            h: 400.0,
        });
        texture_renderer.render();
        swap_buffers();
    }
}
