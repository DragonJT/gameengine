use math::*;
use renderers::{text_renderer::*, *};
use ui::{widget::*, *};

fn main() {
    initialize(2000, 1600);
    cull_back_faces();
    enable_transparency();

    let fontheight = 40.0;
    let mut text_renderer = TextRenderer::new("assets/JetBrainsMono-Medium.ttf", fontheight, 512);
    let mut ui = Widget::screen(Color::green());

    while !window_should_close() {
        let window_size = get_window_size();
        poll_events();
        viewport(0, 0, window_size.x, window_size.y);
        clear_color(1.0, 1.0, 1.0, 1.0);
        clear(BufferBits::Color);

        if is_mouse_down(MouseButton::Left) {
            ui.text_panel(
                "node".to_string(),
                Rect::from_vec2s(get_mouse_position(), Vec2::new(400.0, 250.0)),
            );
        }
        let drawables = ui.draw(&Rect::new(
            0.0,
            0.0,
            window_size.x as f32,
            window_size.y as f32,
        ));

        for d in drawables {
            match d {
                DrawType::DrawRect(drawrect) => match (drawrect.outline) {
                    Some(w) => text_renderer.draw_rect_outline(&drawrect.rect, &drawrect.color, w),
                    None => text_renderer.draw_rect(&drawrect.rect, &drawrect.color),
                },
                DrawType::DrawText(drawtext) => {
                    text_renderer.draw_text(
                        &drawtext.position,
                        &drawtext.text,
                        drawtext.fontscale * fontheight,
                        &drawtext.color,
                    );
                }
                _ => {}
            }
        }
        text_renderer.render();

        swap_buffers();
    }
}
