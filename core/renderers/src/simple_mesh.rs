use math::{mat4::*, vec3::*, *};

pub struct SimpleMesh {
    pub triangles: Vec<Triangle3>,
}

impl SimpleMesh {
    pub fn new() -> Self {
        SimpleMesh { triangles: vec![] }
    }

    pub fn add_face(&mut self, vertices: Vec<Vec3>) {
        for i in 2..vertices.len() {
            self.triangles.push(Triangle3 {
                a: vertices[0],
                b: vertices[i - 1],
                c: vertices[i],
            });
        }
    }

    pub fn add_cube(&mut self, matrix: Mat4) {
        let a = matrix * Vec3::new(-1.0, -1.0, -1.0);
        let b = matrix * Vec3::new(1.0, -1.0, -1.0);
        let c = matrix * Vec3::new(1.0, 1.0, -1.0);
        let d = matrix * Vec3::new(-1.0, 1.0, -1.0);
        let e = matrix * Vec3::new(-1.0, -1.0, 1.0);
        let f = matrix * Vec3::new(1.0, -1.0, 1.0);
        let g = matrix * Vec3::new(1.0, 1.0, 1.0);
        let h = matrix * Vec3::new(-1.0, 1.0, 1.0);

        self.add_face(vec![d, c, b, a]);
        self.add_face(vec![e, f, g, h]);
        self.add_face(vec![a, b, f, e]);
        self.add_face(vec![b, c, g, f]);
        self.add_face(vec![c, d, h, g]);
        self.add_face(vec![d, a, e, h]);
    }
}
