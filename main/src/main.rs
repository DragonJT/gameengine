use std::f32;

use math::{mat4::*, rect::*, *};
use node_script::node::*;
use renderers::{text_renderer::*, *};

fn draw_equilateral_triangle(
    text_renderer: &mut TextRenderer,
    pos: Vec2,
    rotate: f32,
    radius: f32,
    color: Color,
) {
    let mat = Mat4::trs2d(pos, rotate, Vec2::new(radius, radius));
    let tri = Triangle2::new(
        Vec2::new(0.0, -1.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
    );
    text_renderer.draw_triangle(tri.mul(mat), color);
}

fn main() {
    initialize(2000, 1600);
    cull_back_faces();
    enable_transparency();

    let fontheight = 50.0;
    let lineheight = fontheight * 1.3;
    let indentsize = fontheight;
    let mut text_renderer = TextRenderer::new("assets/JetBrainsMono-Medium.ttf", fontheight, 512);
    let mut nodes = Nodes::new();
    let n1 = nodes.add_node(0, "node1");
    nodes.add_node(0, "node2");
    nodes.add_node(n1, "test");
    nodes.add_node(0, "node3");

    while !window_should_close() {
        let window_size = get_window_size();
        poll_events();
        viewport(0, 0, window_size.x, window_size.y);
        clear_color(1.0, 1.0, 1.0, 1.0);
        clear(BufferBits::Color);

        let x = 10.0;
        let mut y = 10.0;
        let tree = nodes.draw_tree();
        for n in tree {
            let pos_triangle = Vec2 {
                x: x + n.depth as f32 * indentsize + fontheight * 0.5,
                y: y + fontheight * 0.7,
            };
            let pos_text = Vec2 {
                x: x + n.depth as f32 * indentsize + fontheight * 1.2,
                y,
            };
            let rect = Rect::new(x, y, 200.0, lineheight);
            if is_mouse_down(MouseButton::Left) && rect.contains(get_mouse_position()) {
                nodes.switch_opened(n.id);
            }
            let angle = if n.opened { f32::consts::PI * 0.5 } else { 0.0 };
            draw_equilateral_triangle(
                &mut text_renderer,
                pos_triangle,
                angle,
                fontheight * 0.3,
                Color::black(),
            );
            text_renderer.draw_text(pos_text, &n.name, fontheight, Color::black());
            y += lineheight;
        }
        text_renderer.render();

        swap_buffers();
    }
}
