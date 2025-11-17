use crate::c;
use crate::helper_functions::*;
use math::*;

pub struct TextRenderer {
    vertices: Vec<f32>,
    vertex_count: u32,
    fontdata: c::FontData,
    atlas_size: i32,
    texture: u32,
    vao: u32,
    vbo: u32,
    program: u32,
    font_height: f32,
}

impl TextRenderer {
    pub fn new(ttf_path: &str, font_height: f32, atlas_size: i32) -> Self {
        let fontdata = create_font_data(ttf_path, font_height, atlas_size, atlas_size);

        let vertex_source = "#version 330 core
        layout (location = 0) in vec2 aPos;
        layout (location = 1) in vec2 aUV;
        layout (location = 2) in vec4 aColor;

        out vec4 color;
        out vec2 uv;
        uniform mat4 view;

        void main()
        {
            color = aColor;
            uv = aUV;
            gl_Position = view * vec4(aPos.x, aPos.y, 0.0, 1.0);
        }";

        let fragment_source = "#version 330 core
        out vec4 FragColor;

        in vec4 color;
        in vec2 uv;
        uniform sampler2D tex;

        void main()
        {
            FragColor = texture(tex, uv).r * color;
        }";
        unsafe {
            let program = initialize_program(vertex_source, fragment_source);
            *fontdata
                .atlas_bitmap
                .add((atlas_size * atlas_size - 1) as usize) = 255u8;
            let texture = c::create_texture(fontdata.atlas_bitmap, atlas_size, atlas_size, 1);
            let vao = c::create_vao();
            let vbo = c::create_vbo();
            c::enable_transparency();
            c::bind_vao(vao);
            c::bind_vbo(vbo);
            c::vertex_attrib_pointer_float(0, 2, 8 * 4, 0);
            c::vertex_attrib_pointer_float(1, 2, 8 * 4, 2 * 4);
            c::vertex_attrib_pointer_float(2, 4, 8 * 4, 4 * 4);
            return TextRenderer {
                vertices: vec![],
                vertex_count: 0,
                fontdata,
                atlas_size,
                texture,
                vao,
                vbo,
                program,
                font_height,
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
            set_matrix(self.program, "view", view.to_f32_ptr());
            c::viewport(0, 0, window_size.x, window_size.y);
            c::clear_color_buffer_bit(1.0, 1.0, 1.0, 1.0);
            c::draw_triangle_arrays(self.vertex_count);
            c::swap_buffers();
            c::poll_events();
            self.clear_vertices();
        }
    }

    pub fn clear_vertices(&mut self) {
        self.vertices.clear();
        self.vertex_count = 0;
    }

    pub fn draw_triangle(&mut self, a: &Vec2, b: &Vec2, c: &Vec2, color: &Color) {
        let vertices = &mut self.vertices;
        let uv = (self.atlas_size as f32 - 0.5) / self.atlas_size as f32;
        self.vertex_count += 3;
        add_vector2(vertices, a.x, a.y);
        add_vector2(vertices, uv, uv);
        add_color(vertices, color);

        add_vector2(vertices, b.x, b.y);
        add_vector2(vertices, uv, uv);
        add_color(vertices, color);

        add_vector2(vertices, c.x, c.y);
        add_vector2(vertices, uv, uv);
        add_color(vertices, color);
    }

    pub fn draw_rect(&mut self, rect: &Rect, color: &Color) {
        self.draw_triangle(
            &Vec2 {
                x: rect.x,
                y: rect.y,
            },
            &Vec2 {
                x: rect.x + rect.w,
                y: rect.y,
            },
            &Vec2 {
                x: rect.x + rect.w,
                y: rect.y + rect.h,
            },
            color,
        );
        self.draw_triangle(
            &Vec2 {
                x: rect.x,
                y: rect.y,
            },
            &Vec2 {
                x: rect.x + rect.w,
                y: rect.y + rect.h,
            },
            &Vec2 {
                x: rect.x,
                y: rect.y + rect.h,
            },
            color,
        );
    }

    pub fn draw_char(&mut self, x: f32, y: f32, c: char, font_height: f32, color: &Color) -> f32 {
        return match get_baked(&self.fontdata, c) {
            Some(baked) => {
                let fontscale = font_height / self.font_height;
                let px = x + baked.xoff * fontscale;
                let py = y + baked.yoff * fontscale + font_height;
                let pw = (baked.x1 - baked.x0) as f32 * fontscale;
                let ph = (baked.y1 - baked.y0) as f32 * fontscale;

                let vertices = &mut self.vertices;
                self.vertex_count += 6;
                add_vector2(vertices, px, py);
                add_vector2(
                    vertices,
                    baked.x0 as f32 / self.atlas_size as f32,
                    baked.y0 as f32 / self.atlas_size as f32,
                );
                add_color(vertices, color);

                add_vector2(vertices, px + pw, py);
                add_vector2(
                    vertices,
                    baked.x1 as f32 / self.atlas_size as f32,
                    baked.y0 as f32 / self.atlas_size as f32,
                );
                add_color(vertices, color);

                add_vector2(vertices, px + pw, py + ph);
                add_vector2(
                    vertices,
                    baked.x1 as f32 / self.atlas_size as f32,
                    baked.y1 as f32 / self.atlas_size as f32,
                );
                add_color(vertices, color);

                add_vector2(vertices, px, py);
                add_vector2(
                    vertices,
                    baked.x0 as f32 / self.atlas_size as f32,
                    baked.y0 as f32 / self.atlas_size as f32,
                );
                add_color(vertices, color);

                add_vector2(vertices, px + pw, py + ph);
                add_vector2(
                    vertices,
                    baked.x1 as f32 / self.atlas_size as f32,
                    baked.y1 as f32 / self.atlas_size as f32,
                );
                add_color(vertices, color);

                add_vector2(vertices, px, py + ph);
                add_vector2(
                    vertices,
                    baked.x0 as f32 / self.atlas_size as f32,
                    baked.y1 as f32 / self.atlas_size as f32,
                );
                add_color(vertices, color);
                baked.xadvance * fontscale
            }
            None => 0.0,
        };
    }

    pub fn draw_text(
        &mut self,
        position: &Vec2,
        text: &str,
        font_height: f32,
        color: &Color,
    ) -> f32 {
        let mut posx = position.x;
        for c in text.chars() {
            posx += self.draw_char(posx, position.y, c, font_height, color);
        }
        return posx - position.x;
    }

    pub fn measure_char(&mut self, c: char, font_height: f32) -> f32 {
        return match get_baked(&self.fontdata, c) {
            Some(baked) => baked.xadvance * font_height / self.font_height,
            None => 0.0,
        };
    }

    pub fn measure_text(&mut self, text: &str, font_height: f32) -> f32 {
        let mut width = 0.0;
        for c in text.chars() {
            width += self.measure_char(c, font_height);
        }
        return width;
    }
}
