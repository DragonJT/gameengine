use std::os::raw::c_char;

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
    pub atlas_bitmap: *mut u8,
    pub baked_chars: *mut BakedChar,
}

unsafe extern "C" {
    pub fn get_char() -> u32;
    pub fn is_key_pressed(key: i32) -> i32;
    pub fn is_mouse_pressed(button: i32) -> i32;
    pub fn is_key_down(key: i32) -> i32;
    pub fn is_key_up(key: i32) -> i32;
    pub fn is_mouse_down(button: i32) -> i32;
    pub fn is_mouse_up(button: i32) -> i32;
    pub fn get_mouse_position() -> math::Vec2;
    pub fn get_mouse_delta() -> math::Vec2;
    pub fn enable_depth_test();
    pub fn disable_depth_test();
    pub fn cull_back_faces();
    pub fn initialize(screen_width: i32, screen_height: i32);
    pub fn initialize_program(vertex_souce: *const c_char, fragment_source: *const c_char) -> u32;
    pub fn create_vao() -> u32;
    pub fn create_vbo() -> u32;
    pub fn set_matrix4(program: u32, name: *const c_char, ptr: *const f32);
    pub fn set_vector3(program: u32, name: *const c_char, x: f32, y: f32, z: f32);
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
    pub fn viewport(x: i32, y: i32, w: i32, h: i32);
    pub fn clear(bits: i32);
    pub fn clear_color(r: f32, g: f32, b: f32, a: f32);
    pub fn draw_triangle_arrays(num_vertices: u32);
    pub fn swap_buffers();
    pub fn poll_events();
    pub fn create_texture() -> u32;
    pub fn tex_image_2d(ptr: *const u8, width: i32, height: i32, channels: i32);
    pub fn generate_mipmap_2d();
    pub fn update_vertices_static(ptr: *const u8, size: u32);
    pub fn update_vertices_dynamic(ptr: *const u8, size: u32);
    pub fn vertex_attrib_pointer_float(id: u32, count: i32, stride: u32, ptr: usize);
    pub fn enable_transparency();
    pub fn window_should_close() -> i32;
    pub fn get_window_size() -> math::Vec2i;
}
