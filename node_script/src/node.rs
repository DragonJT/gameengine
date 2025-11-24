enum Value {
    Node(usize),
}

struct Field {
    name: String,
    value: Value,
}

struct Node {
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
    deleted: bool,
    opened: bool,
}

pub struct Nodes {
    nodes: Vec<Node>,
}

pub struct DrawNode {
    pub id: usize,
    pub depth: usize,
    pub name: String,
    pub opened: bool,
}

impl Nodes {
    pub fn new() -> Nodes {
        let root = Node {
            name: "root".to_string(),
            parent: None,
            children: Vec::new(),
            deleted: false,
            opened: true,
        };
        let nodes: Vec<Node> = vec![root];
        Nodes { nodes }
    }

    fn add_node_to_nodes(&mut self, node: Node) -> usize {
        for i in 1..self.nodes.len() {
            if self.nodes[i].deleted {
                self.nodes[i] = node;
                return i;
            }
        }
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    pub fn add_node(&mut self, parent: usize, name: &str) -> usize {
        let node = Node {
            name: name.to_string(),
            parent: Some(parent),
            children: Vec::new(),
            deleted: false,
            opened: true,
        };
        let id = self.add_node_to_nodes(node);
        self.nodes[parent].children.push(id);
        id
    }

    pub fn switch_opened(&mut self, id: usize) {
        self.nodes[id].opened = !self.nodes[id].opened;
    }

    fn draw_tree_node(&self, depth: usize, id: usize, draw_nodes: &mut Vec<DrawNode>) {
        let opened = self.nodes[id].opened;
        draw_nodes.push(DrawNode {
            id,
            depth,
            name: self.nodes[id].name.to_string(),
            opened,
        });
        if opened {
            for c in &self.nodes[id].children {
                self.draw_tree_node(depth + 1, c.clone(), draw_nodes);
            }
        }
    }

    pub fn draw_tree(&self) -> Vec<DrawNode> {
        let mut draw_nodes: Vec<DrawNode> = vec![];
        self.draw_tree_node(0, 0, &mut draw_nodes);
        draw_nodes
    }
}
