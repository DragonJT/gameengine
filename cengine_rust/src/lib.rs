use math::*;
use std::ffi::CString;

mod c {
    use std::os::raw::c_char;
    #[repr(C)]
    pub struct WindowSize {
        pub x: i32,
        pub y: i32,
    }

    #[repr(C)]
    pub struct BakedChar {
        pub x0: u16,
        pub y0: u16,
        pub x1: u16,
        pub y1: u16,
        pub xoff: f32,
        pub yoff: f32,
        pub xadvance: f32,
    }

    #[repr(C)]
    pub struct FontData {
        pub atlas_bitmap: *const u8,
        pub baked_chars: *mut BakedChar,
    }

    unsafe extern "C" {
        pub fn initialize(screen_width: i32, screen_height: i32);
        pub fn initialize_program(
            vertex_souce: *const c_char,
            fragment_source: *const c_char,
        ) -> u32;
        pub fn create_vao() -> u32;
        pub fn create_vbo() -> u32;
        pub fn set_matrix(program: u32, name: *const c_char, ptr: *const f32);
        pub fn bind_program(program: u32);
        pub fn bind_vao(vao: u32);
        pub fn bind_vbo(vbo: u32);
        pub fn bind_texture(texture: u32);
        pub fn create_font_data(
            ttf_path: *const c_char,
            pixel_height: f32,
            atlas_width: i32,
            atlas_height: i32,
        ) -> FontData;
        pub fn create_texture(ptr: *const u8, width: i32, height: i32, nrChannels: i32) -> u32;
        pub fn update_vertices_static(ptr: *const u8, size: u32);
        pub fn update_vertices_dynamic(ptr: *const u8, size: u32);
        pub fn vertex_attrib_pointer_float(id: u32, count: i32, stride: u32, ptr: usize);
        pub fn enable_transparency();
        pub fn window_should_close() -> i32;
        pub fn render(num_vertices: u32);
        pub fn get_window_size() -> WindowSize;
    }
}

fn update_vertices_static(vertices: &Vec<f32>) {
    let ptr: *const u8 = vertices.as_ptr() as *const u8;
    let size_in_bytes = vertices.len() * std::mem::size_of::<f32>();
    unsafe {
        c::update_vertices_static(ptr, size_in_bytes as u32);
    }
}

fn update_vertices_dynamic(vertices: &Vec<f32>) {
    let ptr: *const u8 = vertices.as_ptr() as *const u8;
    let size_in_bytes = vertices.len() * std::mem::size_of::<f32>();
    unsafe {
        c::update_vertices_dynamic(ptr, size_in_bytes as u32);
    }
}

fn get_cstring(str: &str) -> CString {
    return CString::new(str).unwrap();
}

fn set_matrix(program: u32, name: &str, ptr: *const f32) {
    let a = get_cstring(name);
    unsafe {
        c::set_matrix(program, a.as_ptr(), ptr);
    }
}

fn initialize_program(vertex_source: &str, fragment_source: &str) -> u32 {
    let a = get_cstring(vertex_source);
    let b = get_cstring(fragment_source);
    unsafe {
        return c::initialize_program(a.as_ptr(), b.as_ptr());
    }
}

fn get_baked(fontdata: &c::FontData, c: char) -> Option<&c::BakedChar> {
    let num_chars = 96;
    let baked: &[c::BakedChar] =
        unsafe { std::slice::from_raw_parts(fontdata.baked_chars, num_chars) };
    let code = c as u32;
    if code < 32 || code >= 32 + num_chars as u32 {
        return None;
    }
    Some(&baked[(code - 32) as usize])
}

pub fn create_font_data(
    ttf_path: &str,
    pixel_height: f32,
    atlas_width: i32,
    atlas_height: i32,
) -> c::FontData {
    let a = get_cstring(ttf_path);
    unsafe {
        return c::create_font_data(a.as_ptr(), pixel_height, atlas_width, atlas_height);
    }
}

fn add_vector2(vertices: &mut Vec<f32>, x: f32, y: f32) {
    vertices.push(x);
    vertices.push(y);
}

fn add_color(vertices: &mut Vec<f32>, c: &Color) {
    vertices.push(c.r);
    vertices.push(c.g);
    vertices.push(c.b);
    vertices.push(c.a);
}

pub struct Renderer {
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

impl Renderer {
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
            let texture = c::create_texture(fontdata.atlas_bitmap, atlas_size, atlas_size, 1);
            let vao = c::create_vao();
            let vbo = c::create_vbo();
            c::enable_transparency();
            c::bind_vao(vao);
            c::bind_vbo(vbo);
            c::vertex_attrib_pointer_float(0, 2, 8 * 4, 0);
            c::vertex_attrib_pointer_float(1, 2, 8 * 4, 2 * 4);
            c::vertex_attrib_pointer_float(2, 4, 8 * 4, 4 * 4);
            return Renderer {
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

    pub fn render(&self) {
        unsafe {
            c::bind_vao(self.vao);
            c::bind_vbo(self.vbo);
            update_vertices_dynamic(&self.vertices);
            c::bind_texture(self.texture);
            let window_size = get_window_size();
            let view = Mat4::ortho(
                0.0,
                window_size.x as f32,
                window_size.y as f32,
                0.0,
                -1.0,
                1.0,
            );
            set_matrix(self.program, "view", view.to_f32_ptr());
            c::bind_program(self.program);
            c::bind_vao(self.vao);
            c::render(self.vertex_count);
        }
    }

    pub fn clear_vertices(&mut self) {
        self.vertices.clear();
        self.vertex_count = 0;
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
        x: f32,
        y: f32,
        text: &str,
        font_height: f32,
        color: &Color,
    ) -> f32 {
        let mut posx = x;
        for c in text.chars() {
            posx += self.draw_char(posx, y, c, font_height, color);
        }
        return posx - x;
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

pub fn window_should_close() -> bool {
    unsafe {
        return c::window_should_close() != 0;
    }
}

pub fn initialize(screen_width: i32, screen_height: i32) {
    unsafe {
        c::initialize(screen_width, screen_height);
    }
}

pub fn get_window_size() -> c::WindowSize {
    unsafe { c::get_window_size() }
}
