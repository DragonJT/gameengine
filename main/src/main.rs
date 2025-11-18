use cameras::orbit_camera::*;
use math::{mat4::*, texture::*, vec3::*, *};
use renderers::{lit_renderer::*, text_renderer::*, texture_renderer::*, *};

fn main() {
    initialize(2000, 1600);
    enable_transparency();
    let mut text_renderer = TextRenderer::new("assets/JetBrainsMono-Medium.ttf", 75.0, 512);

    let mut texture_renderer = TextureRenderer::new();
    let mut texture = Texture::new(100, 100, 4);
    texture.draw_circle(50, 50, 50, &[0, 0, 255, 255]);
    texture_renderer.update_texture(&texture);

    let mut orbit_camera = OrbitCamera::new(Vec3::new(0.0, 0.0, 0.0), 5.0, 0.9, 0.1, 100.0);
    let mut basic_lighting = LitRenderer::new();
    let pos = Triangle3 {
        a: Vec3::new(-0.5, -0.5, 0.0),
        b: Vec3::new(0.5, -0.5, 0.0),
        c: Vec3::new(0.0, 0.5, 0.0),
    };
    let normalv = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    let normal = Triangle3 {
        a: normalv.clone(),
        b: normalv.clone(),
        c: normalv.clone(),
    };
    let uv = Triangle2 {
        a: Vec2::new(-0.5, -0.5),
        b: Vec2::new(0.5, -0.5),
        c: Vec2::new(0.0, 0.5),
    };
    basic_lighting.draw_triangle(pos, normal, uv);
    let mut texture = Texture::new(1, 1, 4);
    texture.set_pixel(0, 0, &[255, 0, 0, 255]);
    basic_lighting.update_texture(&texture);

    while !window_should_close() {
        let window_size = get_window_size();
        poll_events();
        viewport(0, 0, window_size.x, window_size.y);
        clear_color_buffer_bit(1.0, 1.0, 1.0, 1.0);

        let rotate_speed = 0.2;
        if is_key_pressed(Key::Left) {
            orbit_camera.rotate(rotate_speed, 0.0);
        }
        if is_key_pressed(Key::Right) {
            orbit_camera.rotate(-rotate_speed, 0.0);
        }
        if is_key_pressed(Key::Up) {
            orbit_camera.rotate(0.0, -rotate_speed);
        }
        if is_key_pressed(Key::Down) {
            orbit_camera.rotate(0.0, rotate_speed);
        }
        let model = Mat4::IDENTITY;
        let view = orbit_camera.view_matrix();
        let projection =
            orbit_camera.projection_matrix(window_size.x as f32 / window_size.y as f32);
        let light_pos = Vec3 {
            x: 3.0,
            y: 3.0,
            z: 3.0,
        };
        let light_color = Color::white();
        basic_lighting.render(
            &model,
            &view,
            &projection,
            &orbit_camera.position(),
            &light_pos,
            &light_color,
        );

        text_renderer.draw_rect(
            &Rect {
                x: 100.0,
                y: 100.0,
                w: 500.0,
                h: 500.0,
            },
            &Color::red(),
        );
        text_renderer.draw_text(
            &Vec2 { x: 100.0, y: 100.0 },
            "HelloWorld",
            300.0,
            &Color::white(),
        );
        text_renderer.render();

        texture_renderer.draw_full_texture(&Rect {
            x: 400.0,
            y: 400.0,
            w: 400.0,
            h: 400.0,
        });
        texture_renderer.render();
        swap_buffers();
    }
}
