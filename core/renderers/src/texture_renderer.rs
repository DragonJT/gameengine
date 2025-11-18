use crate::c;
use crate::helper_functions::*;
use math::{mat4::*, texture::*, *};

pub struct TextureRenderer {
    vertices: Vec<f32>,
    vertex_count: u32,
    texture: u32,
    vao: u32,
    vbo: u32,
    program: u32,
}

impl TextureRenderer {
    pub fn new() -> Self {
        let vertex_source = "#version 330 core
        layout (location = 0) in vec2 aPos;
        layout (location = 1) in vec2 aUV;

        out vec2 uv;
        uniform mat4 view;

        void main()
        {
            uv = aUV;
            gl_Position = view * vec4(aPos.x, aPos.y, 0.0, 1.0);
        }";

        let fragment_source = "#version 330 core
        out vec4 FragColor;

        in vec2 uv;
        uniform sampler2D tex;

        void main()
        {
            FragColor = texture(tex, uv);
        }";
        unsafe {
            let program = initialize_program(vertex_source, fragment_source);
            let texture = c::create_texture();
            let vao = c::create_vao();
            let vbo = c::create_vbo();
            c::bind_vao(vao);
            c::bind_vbo(vbo);
            c::vertex_attrib_pointer_float(0, 2, 4 * 4, 0);
            c::vertex_attrib_pointer_float(1, 2, 4 * 4, 2 * 4);
            return TextureRenderer {
                vertices: vec![],
                vertex_count: 0,
                texture,
                vao,
                vbo,
                program,
            };
        }
    }

    pub fn render(&mut self) {
        unsafe {
            c::bind_program(self.program);
            c::bind_vao(self.vao);
            c::bind_vbo(self.vbo);
            update_vertices_dynamic(&self.vertices);
            c::bind_texture(self.texture);
            let window_size = c::get_window_size();
            let view = Mat4::ortho(
                0.0,
                window_size.x as f32,
                window_size.y as f32,
                0.0,
                -1.0,
                1.0,
            );
            set_matrix4(self.program, "view", view.to_f32_ptr());
            c::draw_triangle_arrays(self.vertex_count);
            self.clear_vertices();
        }
    }

    pub fn update_texture(&mut self, texture: &Texture) {
        let ptr = texture.data.as_ptr();
        unsafe {
            c::bind_texture(self.texture);
            c::tex_image_2d(ptr, texture.width, texture.height, texture.channels);
        }
    }

    pub fn clear_vertices(&mut self) {
        self.vertices.clear();
        self.vertex_count = 0;
    }

    pub fn draw_triangle(&mut self, t: &Triangle2, uv: &Triangle2) {
        self.vertex_count += 3;
        self.vertices.push(t.a.x);
        self.vertices.push(t.a.y);
        self.vertices.push(uv.a.x);
        self.vertices.push(uv.a.y);

        self.vertices.push(t.b.x);
        self.vertices.push(t.b.y);
        self.vertices.push(uv.b.x);
        self.vertices.push(uv.b.y);

        self.vertices.push(t.c.x);
        self.vertices.push(t.c.y);
        self.vertices.push(uv.c.x);
        self.vertices.push(uv.c.y);
    }

    pub fn draw_texture(&mut self, rect: &Rect, uv: &Rect) {
        self.draw_triangle(&rect.tri1(), &uv.tri1());
        self.draw_triangle(&rect.tri2(), &uv.tri2());
    }

    pub fn draw_full_texture(&mut self, rect: &Rect) {
        let uv = Rect {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        };
        self.draw_triangle(&rect.tri1(), &uv.tri1());
        self.draw_triangle(&rect.tri2(), &uv.tri2());
    }
}
