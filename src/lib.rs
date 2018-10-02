#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

impl Vertex {
    pub fn new(p: glm::Vec3) -> Vertex {
        Vertex {
            position: *p.as_ref(),
        }
    }
}

pub struct Object {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Object {
    pub fn square(origin: glm::Vec3, size: f32) -> Object {
        Object {
            vertices: vec![
                Vertex::new(origin),
                Vertex::new(origin + glm::vec3(size, 0.0, 0.0)),
                Vertex::new(origin + glm::vec3(0.0, size, 0.0)),
                Vertex::new(origin + glm::vec3(size, size, 0.0)),
            ],
            indices: vec![0, 1, 2, 1, 2, 3],
        }
    }

    pub fn triangle(a: glm::Vec3, b: glm::Vec3, c: glm::Vec3) -> Object {
        Object {
            vertices: vec![Vertex::new(a), Vertex::new(b), Vertex::new(c)],
            indices: vec![0, 1, 2],
        }
    }

    pub fn vertex_buffer(&self, display: &glium::Display) -> glium::VertexBuffer<Vertex> {
        glium::VertexBuffer::new(display, &self.vertices).unwrap()
    }

    pub fn index_buffer(&self, display: &glium::Display) -> glium::IndexBuffer<u16> {
        glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &self.indices,
        ).unwrap()
    }
}
