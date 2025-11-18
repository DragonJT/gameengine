use crate::c;
use crate::helper_functions::*;
use math::{mat4::*, texture::*, vec3::*, *};

pub struct LitRenderer {
    vertices: Vec<f32>,
    vertex_count: u32,
    texture: u32,
    vao: u32,
    vbo: u32,
    program: u32,
}

impl LitRenderer {
    pub fn new() -> Self {
        let vertex_source = "#version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aNormal;
        layout (location = 2) in vec2 aUV;

        out vec3 FragPos;
        out vec3 Normal;
        out vec2 uv;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main()
        {
            uv = aUV;
            FragPos = vec3(model * vec4(aPos, 1.0));
            Normal = mat3(transpose(inverse(model))) * aNormal;

            gl_Position = projection * view * vec4(FragPos, 1.0);
        }";

        let fragment_source = "#version 330 core
        out vec4 FragColor;

        in vec3 Normal;
        in vec3 FragPos;
        in vec2 uv;

        uniform sampler2D tex;
        uniform vec3 lightPos;
        uniform vec3 viewPos;
        uniform vec3 lightColor;

        void main()
        {
            // ambient
            float ambientStrength = 0.1;
            vec3 ambient = ambientStrength * lightColor;

            // diffuse
            vec3 norm = normalize(Normal);
            vec3 lightDir = normalize(lightPos - FragPos);
            float diff = max(dot(norm, lightDir), 0.0);
            vec3 diffuse = diff * lightColor;

            // specular
            float specularStrength = 0.5;
            vec3 viewDir = normalize(viewPos - FragPos);
            vec3 reflectDir = reflect(-lightDir, norm);
            float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
            vec3 specular = specularStrength * spec * lightColor;

            vec4 objectColor = texture(tex, uv);
            vec4 result = vec4(ambient + diffuse + specular, 1.0) * objectColor;
            FragColor = result;
        } ";
        unsafe {
            let program = initialize_program(vertex_source, fragment_source);
            let texture = c::create_texture();
            let vao = c::create_vao();
            let vbo = c::create_vbo();
            c::bind_vao(vao);
            c::bind_vbo(vbo);
            c::vertex_attrib_pointer_float(0, 3, 8 * 4, 0);
            c::vertex_attrib_pointer_float(1, 3, 8 * 4, 3 * 4);
            c::vertex_attrib_pointer_float(2, 2, 8 * 4, 6 * 4);

            return LitRenderer {
                vertices: vec![],
                vertex_count: 0,
                texture,
                vao,
                vbo,
                program,
            };
        }
    }

    pub fn render(
        &mut self,
        model: &Mat4,
        view: &Mat4,
        projection: &Mat4,
        view_pos: &Vec3,
        light_pos: &Vec3,
        light_color: &Color,
    ) {
        unsafe {
            c::bind_program(self.program);
            c::bind_vao(self.vao);
            c::bind_vbo(self.vbo);
            update_vertices_dynamic(&self.vertices);
            c::bind_texture(self.texture);
            set_matrix4(self.program, "model", model.to_f32_ptr());
            set_matrix4(self.program, "view", view.to_f32_ptr());
            set_matrix4(self.program, "projection", projection.to_f32_ptr());
            set_vector3(self.program, "viewPos", view_pos);
            set_vector3(self.program, "lightPos", light_pos);
            set_vector3(self.program, "lightColor", &light_color.to_vec3());
            c::draw_triangle_arrays(self.vertex_count);
        }
    }

    pub fn draw_triangle(&mut self, pos: Triangle3, normal: Triangle3, uv: Triangle2) {
        let vertices = &mut self.vertices;
        self.vertex_count += 3;
        add_vector3(vertices, &pos.a);
        add_vector3(vertices, &normal.a);
        add_vector2(vertices, &uv.a);

        add_vector3(vertices, &pos.b);
        add_vector3(vertices, &normal.b);
        add_vector2(vertices, &uv.b);

        add_vector3(vertices, &pos.c);
        add_vector3(vertices, &normal.c);
        add_vector2(vertices, &uv.c);
    }

    pub fn update_texture(&mut self, texture: &Texture) {
        let ptr = texture.data.as_ptr();
        unsafe {
            c::bind_texture(self.texture);
            c::tex_image_2d(ptr, texture.width, texture.height, texture.channels);
        }
    }
}
