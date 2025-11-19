use math::{rect::*, *};

#[derive(Clone, Copy)]
pub enum Element {
    Textbox(usize),
    Node(usize),
}

pub struct Node {
    pub position: Vec2,
    pub size: Vec2,
    pub name: String,
    pub elements: Vec<Element>,
}

pub struct Label {
    pub text: String,
    pub position: Vec2,
}

pub struct Textbox {
    pub label: Option<Label>,
    pub position: Vec2,
    pub size: Vec2,
    pub value: String,
}

pub struct Style {
    pub node_border: f32,
    pub fontheight: f32,
    pub lineheight: f32,
    pub label_color: Color,
    pub node_header_color: Color,
}

pub struct UI {
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
                node_header_color: Color::new(0.2, 0.4, 0.2, 1.0),
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
            position,
            size,
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

    fn set_position(&mut self, element: Element, position: Vec2) {
        match element {
            Element::Textbox(t) => self.textboxes[t].position = position,
            Element::Node(n) => self.nodes[n].position = position,
        }
    }

    fn set_size(&mut self, element: Element, size: Vec2) {
        match element {
            Element::Textbox(t) => self.textboxes[t].size = size,
            Element::Node(n) => self.nodes[n].size = size,
        }
    }

    fn get_position(&mut self, element: Element) -> Vec2 {
        match element {
            Element::Textbox(t) => self.textboxes[t].position,
            Element::Node(n) => self.nodes[n].position,
        }
    }

    fn get_size(&mut self, element: Element) -> Vec2 {
        match element {
            Element::Textbox(t) => self.textboxes[t].size,
            Element::Node(n) => self.nodes[n].size,
        }
    }

    fn get_rect(&mut self, element: Element) -> Rect {
        let pos = self.get_position(element);
        let size = self.get_size(element);
        Rect::from_vec2s(pos, size)
    }

    fn layout_elements(&mut self, elements: &Vec<Element>, rect: Rect) {
        let x = rect.x;
        let mut y = rect.y;
        let lh = self.style.lineheight;
        let w = rect.w - lh;
        for e in elements {
            self.set_label_position(e.clone(), Vec2::new(x + lh, y));
            y += lh;
            self.set_position(e.clone(), Vec2::new(x + lh, y));
            self.set_size(e.clone(), Vec2::new(w, self.style.fontheight));
            y += lh;
        }
    }

    fn layout_node(&mut self, node: usize) {
        let elements_rect = self
            .get_rect(Element::Node(node))
            .lower_top(self.style.lineheight)
            .expand(-self.style.node_border);
        let elements = self.nodes[node].elements.clone();
        self.layout_elements(&elements, elements_rect);
    }

    pub fn add_node(&mut self, position: Vec2, name: &str, inputs: Vec<&str>) -> Element {
        let size = Vec2::new(
            500.0,
            inputs.len() as f32 * self.style.lineheight * 2.0 + self.style.lineheight + 40.0,
        );
        let mut elements: Vec<Element> = vec![];
        for i in inputs {
            elements.push(self.add_textbox(Vec2::new(0.0, 0.0), Some(i.to_string())));
        }

        self.nodes.push(Node {
            position,
            size,
            name: name.to_string(),
            elements,
        });
        let id = self.nodes.len() - 1;
        self.layout_node(id);
        Element::Node(id)
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
            let rect = self.get_rect(Element::Textbox(i));
            if rect.contains(&mousepos) {
                self.selected = Some(Element::Textbox(i));
                return;
            }
        }
        for i in 0..self.nodes.len() {
            let rect = self.get_rect(Element::Node(i));
            if rect.contains(&mousepos) {
                self.dragging = Some(Element::Node(i));
                return;
            }
        }
    }

    pub fn mousedrag(&mut self, mousedelta: Vec2) {
        match self.dragging {
            Some(d) => match d {
                Element::Node(n) => {
                    let position = self.get_position(d);
                    self.set_position(d, position + mousedelta);
                    self.layout_node(n);
                }
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
