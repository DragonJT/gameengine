use math::*;
mod c;
mod helper_functions;
pub mod lit_renderer;
pub mod text_renderer;
pub mod texture_renderer;

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

pub fn get_window_size() -> Vec2i {
    unsafe { c::get_window_size() }
}

pub fn poll_events() {
    unsafe {
        c::poll_events();
    }
}

pub fn viewport(x: i32, y: i32, w: i32, h: i32) {
    unsafe {
        c::viewport(x, y, w, h);
    }
}

pub fn clear_color_buffer_bit(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        c::clear_color_buffer_bit(r, g, b, a);
    }
}

pub fn swap_buffers() {
    unsafe {
        c::swap_buffers();
    }
}

pub fn enable_transparency() {
    unsafe {
        c::enable_transparency();
    }
}
