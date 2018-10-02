#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
}
implement_vertex!(Vertex, pos);

impl Vertex {
    pub fn new(p: glm::Vec3) -> Vertex {
        Vertex { pos: *p.as_ref() }
    }
}

pub fn make_triangle(a: glm::Vec3, b: glm::Vec3, c: glm::Vec3) -> Vec<Vertex> {
    vec![
        Vertex { pos: *a.as_ref() },
        Vertex { pos: *b.as_ref() },
        Vertex { pos: *c.as_ref() },
    ]
}

pub fn make_square(origin: glm::Vec3, size: f32) -> (Vec<Vertex>, Vec<u16>) {
    (
        vec![
            Vertex::new(origin),
            Vertex::new(origin + glm::vec3(size, 0.0, 0.0)),
            Vertex::new(origin + glm::vec3(0.0, size, 0.0)),
            Vertex::new(origin + glm::vec3(size, size, 0.0)),
        ],
        vec![0, 1, 2, 1, 2, 3],
    )
}
