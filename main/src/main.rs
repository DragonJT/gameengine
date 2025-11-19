use math::{rect::*, *};
use renderers::{text_renderer::*, *};

enum Element {
    None,
    Textbox(usize),
    Node(usize),
}

struct Node {
    pub rect: Rect,
    pub elements: Vec<Element>,
}

struct TextBox {
    pub rect: Rect,
    pub value: String,
}

struct UI {
    pub fontheight: f32,
    pub lineheight: f32,
    pub textboxes: Vec<TextBox>,
    pub nodes: Vec<Node>,
    pub dragging: Element,
    pub selected: Element,
}

impl UI {
    pub fn new(fontheight: f32) -> Self {
        UI {
            fontheight,
            lineheight: fontheight * 1.3,
            textboxes: vec![],
            nodes: vec![],
            dragging: Element::None,
            selected: Element::None,
        }
    }

    pub fn add_textbox(&mut self, position: Vec2, value: String) -> Element {
        let size = Vec2::new(400.0, self.fontheight);
        self.textboxes.push(TextBox {
            rect: Rect::from_vec2s(position, size),
            value,
        });
        Element::Textbox(self.textboxes.len() - 1)
    }

    pub fn add_node(&mut self, position: Vec2, inputs: Vec<&str>) -> Element {
        let size = Vec2::new(500.0, inputs.len() as f32 * self.lineheight);
        let mut elements: Vec<Element> = vec![];
        let x = position.x;
        let mut y = position.y;
        for i in inputs {
            elements.push(self.add_textbox(Vec2::new(x, y), i.to_string()));
            y += self.lineheight;
        }
        self.nodes.push(Node {
            rect: Rect::from_vec2s(position, size),
            elements,
        });
        Element::Node(self.nodes.len() - 1)
    }

    pub fn mousedown(&mut self, mousepos: Vec2) {
        self.selected = Element::None;
        for i in 0..self.textboxes.len() {
            let t = &self.textboxes[i];
            if t.rect.contains(&mousepos) {
                self.selected = Element::Textbox(i);
                return;
            }
        }
        for i in 0..self.nodes.len() {
            let n = &self.nodes[i];
            if n.rect.contains(&mousepos) {
                self.dragging = Element::Node(i);
                return;
            }
        }
    }

    pub fn mousedrag(&mut self, mousedelta: Vec2) {
        match self.dragging {
            Element::Node(n) => {
                self.nodes[n.clone()].rect = self.nodes[n.clone()].rect + mousedelta;
            }
            Element::Textbox(t) => {}
            Element::None => {}
        };
    }

    pub fn mouseup(&mut self) {
        self.dragging = Element::None;
    }

    pub fn is_textbox_selected(&mut self, i: usize) -> bool {
        match self.selected {
            Element::Textbox(t) => t == i,
            _ => false,
        }
    }
}

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

        if is_key_down(Key::Enter) {
            ui.add_node(get_mouse_position(), vec!["test", "bob"]);
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

        for n in &ui.nodes {
            text_renderer.draw_rect(&n.rect, &Color::new(0.9, 0.9, 0.9, 1.0));
            text_renderer.draw_rect_outline(&n.rect, &Color::black(), 2.0);
        }
        for i in 0..ui.textboxes.len() {
            let color = if ui.is_textbox_selected(i) {
                Color::red()
            } else {
                Color::black()
            };
            let t = &ui.textboxes[i];
            text_renderer.draw_rect(&t.rect, &Color::white());
            text_renderer.draw_rect_outline(&t.rect, &color, 2.0);
            text_renderer.draw_text(&t.rect.topleft(), &t.value, fontheight, &Color::black());
        }
        text_renderer.render();

        swap_buffers();
    }
}
