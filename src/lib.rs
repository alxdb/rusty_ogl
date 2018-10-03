#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    colour: [f32; 3],
}
implement_vertex!(Vertex, position, colour);

pub struct Colour(pub f32, pub f32, pub f32);

impl Vertex {
    pub fn new(p: &glm::Vec3) -> Vertex {
        Vertex {
            position: *p.as_ref(),
            colour: [0.2, 0.1, 0.0],
        }
    }

    pub fn pos(&self) -> glm::Vec3 {
        glm::vec3(self.position[0], self.position[1], self.position[2])
    }

    pub fn set_colour(&mut self, c: &Colour) {
        self.colour = [c.0, c.1, c.2];
    }
}

pub struct Object {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Object {
    pub fn square(origin: &glm::Vec3, size: f32) -> Object {
        Object {
            vertices: vec![
                Vertex::new(origin),
                Vertex::new(&(origin + glm::vec3(size, 0.0, 0.0))),
                Vertex::new(&(origin + glm::vec3(0.0, size, 0.0))),
                Vertex::new(&(origin + glm::vec3(size, size, 0.0))),
            ],
            indices: vec![0, 1, 2, 1, 2, 3],
        }
    }

    pub fn triangle(a: &glm::Vec3, b: &glm::Vec3, c: &glm::Vec3) -> Object {
        Object {
            vertices: vec![Vertex::new(a), Vertex::new(b), Vertex::new(c)],
            indices: vec![0, 1, 2],
        }
    }

    pub fn update<F: Fn(&Vertex) -> Vertex>(&mut self, f: F) {
        for v in &mut self.vertices {
            *v = f(&v);
        }
    }

    pub fn transform<F: Fn(&Vertex) -> Vertex>(&self, f: F) -> Object {
        let mut new_v = Vec::new();
        for v in &self.vertices {
            new_v.push(f(&v));
        }
        Object {
            vertices: new_v,
            indices: self.indices.clone(),
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
