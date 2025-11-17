use math::*;
mod c;
mod helper_functions;
pub mod text_renderer;

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
