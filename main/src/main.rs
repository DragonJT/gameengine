mod compile;
mod visual_scripting;
use compile::*;
use math::{rect::Rect, *};
use renderers::{text_renderer::*, *};
use visual_scripting::*;

fn main() {
    initialize(2000, 1600);
    cull_back_faces();
    enable_transparency();

    let fontheight = 40.0;
    let mut text_renderer = TextRenderer::new("assets/JetBrainsMono-Medium.ttf", fontheight, 512);
    let mut ui = UI::new(fontheight);

    while !window_should_close() {
        let window_size = get_window_size();
        poll_events();
        viewport(0, 0, window_size.x, window_size.y);
        clear_color(1.0, 1.0, 1.0, 1.0);
        clear(BufferBits::Color);

        if is_control() {
            if is_key_down(Key::R) {
                run(&mut ui);
            }
        } else {
            if is_key_down(Key::Enter) {
                ui.add_node(get_mouse_position(), "printf", vec!["text"]);
            }
            if is_key_down(Key::Backspace) {
                ui.backspace();
            }
            match get_char() {
                Some(c) => ui.add_char(c),
                None => {}
            }
            if is_mouse_down(MouseButton::Left) {
                ui.mousedown(get_mouse_position());
            }
            if is_mouse_pressed(MouseButton::Left) {
                ui.mousedrag(get_mouse_delta());
            }
            if is_mouse_up(MouseButton::Left) {
                ui.mouseup();
            }
        }

        for n in &ui.nodes {
            let rect = Rect::from_vec2s(n.position, n.size);
            text_renderer.draw_rect(&rect, &Color::new(0.9, 0.9, 0.9, 1.0));
            text_renderer.draw_rect_outline(&rect, &Color::black(), 2.0);
            text_renderer.draw_text(
                &n.position,
                &n.name,
                ui.style.fontheight,
                &ui.style.node_header_color,
            );
        }
        for i in 0..ui.textboxes.len() {
            let color = if ui.is_textbox_selected(i) {
                Color::red()
            } else {
                Color::black()
            };
            let t = &ui.textboxes[i];
            match &t.label {
                Some(label) => {
                    text_renderer.draw_text(
                        &label.position,
                        &label.text,
                        ui.style.fontheight,
                        &ui.style.label_color,
                    );
                }
                None => {}
            }
            let rect = Rect::from_vec2s(t.position, t.size);
            text_renderer.draw_rect(&rect, &Color::white());
            text_renderer.draw_rect_outline(&rect, &color, 2.0);
            text_renderer.draw_text(&t.position, &t.value, fontheight, &Color::black());
        }
        text_renderer.render();

        swap_buffers();
    }
}
