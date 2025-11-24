use crate::c;
use math::{vec3::*, *};
use std::ffi::CString;

pub fn update_vertices_static(vertices: &Vec<f32>) {
    let ptr: *const u8 = vertices.as_ptr() as *const u8;
    let size_in_bytes = vertices.len() * std::mem::size_of::<f32>();
    unsafe {
        c::update_vertices_static(ptr, size_in_bytes as u32);
    }
}

pub fn update_vertices_dynamic(vertices: &Vec<f32>) {
    let ptr: *const u8 = vertices.as_ptr() as *const u8;
    let size_in_bytes = vertices.len() * std::mem::size_of::<f32>();
    unsafe {
        c::update_vertices_dynamic(ptr, size_in_bytes as u32);
    }
}

pub fn get_cstring(str: &str) -> CString {
    return CString::new(str).unwrap();
}

pub fn set_matrix4(program: u32, name: &str, ptr: *const f32) {
    let a = get_cstring(name);
    unsafe {
        c::set_matrix4(program, a.as_ptr(), ptr);
    }
}

pub fn set_vector3(program: u32, name: &str, v: Vec3) {
    let a = get_cstring(name);
    unsafe {
        c::set_vector3(program, a.as_ptr(), v.x, v.y, v.z);
    }
}

pub fn initialize_program(vertex_source: &str, fragment_source: &str) -> u32 {
    let a = get_cstring(vertex_source);
    let b = get_cstring(fragment_source);
    unsafe {
        return c::initialize_program(a.as_ptr(), b.as_ptr());
    }
}

pub fn get_baked(fontdata: &c::FontData, c: char) -> Option<&c::BakedChar> {
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

pub fn add_vector2(vertices: &mut Vec<f32>, v: Vec2) {
    vertices.push(v.x);
    vertices.push(v.y);
}

pub fn add_vector3(vertices: &mut Vec<f32>, v: Vec3) {
    vertices.push(v.x);
    vertices.push(v.y);
    vertices.push(v.z);
}

pub fn add_color(vertices: &mut Vec<f32>, c: Color) {
    vertices.push(c.r);
    vertices.push(c.g);
    vertices.push(c.b);
    vertices.push(c.a);
}
