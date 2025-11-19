use math::{rect::*, *};
use renderers::{text_renderer::*, *};

struct Node {
    pub rect: Rect,
}

struct Nodes {
    pub nodes: Vec<Node>,
    pub dragging: Option<usize>,
}

impl Nodes {
    pub fn new() -> Self {
        Nodes {
            nodes: vec![],
            dragging: None,
        }
    }
}

fn main() {
    initialize(2000, 1600);
    cull_back_faces();
    enable_transparency();

    let fontheight = 40.0;
    let mut text_renderer = TextRenderer::new("assets/JetBrainsMono-Medium.ttf", fontheight, 512);
    let mut nodes = Nodes::new();

    while !window_should_close() {
        let window_size = get_window_size();
        poll_events();
        viewport(0, 0, window_size.x, window_size.y);
        clear_color(1.0, 1.0, 1.0, 1.0);
        clear(BufferBits::Color);

        if is_key_down(Key::Enter) {
            nodes.nodes.push(Node {
                rect: Rect::from_vec2s(get_mouse_position(), Vec2::new(400.0, 100.0)),
            });
        }

        if is_mouse_down(MouseButton::Left) {
            let mousepos = get_mouse_position();
            for i in 0..nodes.nodes.len() {
                let n = &nodes.nodes[i];
                if n.rect.contains(&mousepos) {
                    nodes.dragging = Some(i);
                    break;
                }
            }
        }
        if is_mouse_pressed(MouseButton::Left) {
            match &nodes.dragging {
                Some(d) => {
                    nodes.nodes[d.clone()].rect = nodes.nodes[d.clone()].rect + get_mouse_delta();
                }
                None => {}
            }
        }
        if is_mouse_up(MouseButton::Left) {
            match &nodes.dragging {
                Some(_) => {
                    nodes.dragging = None;
                }
                None => {}
            }
        }

        for n in &nodes.nodes {
            text_renderer.draw_rect(&n.rect, &Color::blue());
            text_renderer.draw_rect_outline(&n.rect, &Color::black(), 2.0);
            text_renderer.draw_text(&n.rect.topleft(), "Node", fontheight, &Color::white());
        }
        text_renderer.render();

        swap_buffers();
    }
}
