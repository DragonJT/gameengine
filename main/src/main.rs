use math::{rect::*, *};
use renderers::{text_renderer::*, *};

#[derive(Clone, Copy)]
enum Element {
    Textbox(usize),
    Node(usize),
}

struct Node {
    pub rect: Rect,
    pub elements: Vec<Element>,
}

struct Label {
    pub text: String,
    pub position: Vec2,
}

struct Textbox {
    pub label: Option<Label>,
    pub rect: Rect,
    pub value: String,
}

struct Style {
    pub node_border: f32,
    pub fontheight: f32,
    pub lineheight: f32,
    pub label_color: Color,
}

struct UI {
    pub style: Style,
    pub textboxes: Vec<Textbox>,
    pub nodes: Vec<Node>,
    pub dragging: Option<Element>,
    pub selected: Option<Element>,
}

impl UI {
    pub fn new(fontheight: f32) -> Self {
        UI {
            style: Style {
                fontheight: fontheight * 1.2,
                lineheight: fontheight * 1.6,
                node_border: 20.0,
                label_color: Color::new(0.3, 0.3, 0.3, 1.0),
            },
            textboxes: vec![],
            nodes: vec![],
            dragging: None,
            selected: None,
        }
    }

    pub fn add_textbox(&mut self, position: Vec2, label_text: Option<String>) -> Element {
        let size = Vec2::new(400.0, self.style.fontheight);
        let label = match label_text {
            Some(text) => Some(Label {
                text,
                position: Vec2::new(0.0, 0.0),
            }),
            None => None,
        };
        self.textboxes.push(Textbox {
            label,
            rect: Rect::from_vec2s(position, size),
            value: "".to_string(),
        });
        Element::Textbox(self.textboxes.len() - 1)
    }

    pub fn get_editable_text(&mut self, element: Element) -> Option<String> {
        match element {
            Element::Node(_) => None,
            Element::Textbox(t) => Some(self.textboxes[t.clone()].value.clone()),
        }
    }

    pub fn set_editable_text(&mut self, element: Element, text: String) {
        match element {
            Element::Node(_) => {}
            Element::Textbox(t) => self.textboxes[t].value = text,
        }
    }

    fn set_label_position(&mut self, element: Element, position: Vec2) {
        match element {
            Element::Textbox(textbox) => match &self.textboxes[textbox].label {
                Some(label) => {
                    self.textboxes[textbox].label = Some(Label {
                        position,
                        text: label.text.clone(),
                    });
                }
                None => {}
            },
            Element::Node(_) => {}
        }
    }

    fn set_rect(&mut self, element: Element, rect: Rect) {
        match element {
            Element::Textbox(tb) => {
                self.textboxes[tb].rect = rect;
            }
            _ => {}
        }
    }

    fn layout_elements(&mut self, elements: &Vec<Element>, rect: Rect) {
        let x = rect.x;
        let mut y = rect.y;
        let w = rect.w;
        for e in elements {
            self.set_label_position(e.clone(), Vec2::new(x, y));
            y += self.style.lineheight;
            self.set_rect(e.clone(), Rect::new(x, y, w, self.style.fontheight));
            y += self.style.lineheight;
        }
    }

    pub fn add_node(&mut self, position: Vec2, inputs: Vec<&str>) -> Element {
        let size = Vec2::new(
            500.0,
            inputs.len() as f32 * self.style.lineheight * 2.0 + 40.0,
        );
        let rect = Rect::from_vec2s(position, size);
        let mut elements: Vec<Element> = vec![];

        for i in inputs {
            elements.push(self.add_textbox(Vec2::new(0.0, 0.0), Some(i.to_string())));
        }
        let elements_rect = rect.expand(-self.style.node_border);
        self.layout_elements(&elements, elements_rect);

        self.nodes.push(Node { rect, elements });
        Element::Node(self.nodes.len() - 1)
    }

    pub fn get_children(&mut self, element: Element) -> Vec<Element> {
        return match element {
            Element::Textbox(_) => vec![],
            Element::Node(n) => self.nodes[n].elements.clone(),
        };
    }

    pub fn mousedown(&mut self, mousepos: Vec2) {
        self.selected = None;
        for i in 0..self.textboxes.len() {
            let t = &self.textboxes[i];
            if t.rect.contains(&mousepos) {
                self.selected = Some(Element::Textbox(i));
                return;
            }
        }
        for i in 0..self.nodes.len() {
            let n = &self.nodes[i];
            if n.rect.contains(&mousepos) {
                self.dragging = Some(Element::Node(i));
                return;
            }
        }
    }

    pub fn mousedrag(&mut self, mousedelta: Vec2) {
        match self.dragging {
            Some(d) => match d {
                Element::Node(n) => match self.dragging {
                    Some(d) => {
                        let rect = self.nodes[n.clone()].rect + mousedelta;
                        self.nodes[n.clone()].rect = rect;
                        let elements = self.get_children(d);
                        self.layout_elements(
                            &elements.clone(),
                            rect.expand(-self.style.node_border),
                        );
                    }
                    None => {}
                },
                Element::Textbox(_) => {}
            },
            None => {}
        };
    }

    pub fn add_char(&mut self, c: char) {
        match self.selected {
            Some(s) => match self.get_editable_text(s) {
                Some(mut text) => {
                    text.push(c);
                    self.set_editable_text(s, text);
                }
                None => {}
            },
            None => {}
        }
    }

    pub fn backspace(&mut self) {
        match self.selected {
            Some(s) => match self.get_editable_text(s) {
                Some(mut text) => {
                    text.pop();
                    self.set_editable_text(s, text);
                }
                None => {}
            },
            None => {}
        }
    }

    pub fn mouseup(&mut self) {
        self.dragging = None;
    }

    pub fn is_textbox_selected(&mut self, i: usize) -> bool {
        match self.selected {
            Some(s) => match s {
                Element::Textbox(t) => t == i,
                _ => false,
            },
            None => false,
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
            text_renderer.draw_rect(&t.rect, &Color::white());
            text_renderer.draw_rect_outline(&t.rect, &color, 2.0);
            text_renderer.draw_text(&t.rect.topleft(), &t.value, fontheight, &Color::black());
        }
        text_renderer.render();

        swap_buffers();
    }
}
