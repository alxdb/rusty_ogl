use glium;
use glm;
use shaders::Shader;

mod primitives;
use self::primitives::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    colour: [f32; 3],
}
implement_vertex!(Vertex, position, colour);

impl Vertex {
    pub fn new(position: &glm::Vec3, colour: &glm::Vec3) -> Vertex {
        Vertex {
            position: *position.as_ref(),
            colour: *colour.as_ref(),
        }
    }

    pub fn update(&mut self, position: &glm::Vec3, colour: &glm::Vec3) {
        self.position = *position.as_ref();
        self.colour = *colour.as_ref();
    }
}

pub struct Buffers {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<Index>,
}

pub struct Object {
    shader: Shader,
    primitive: Primitive,
    vertices: Vec<Vertex>,
}

impl Object {
    pub fn new<F: Fn(&glm::Vec3) -> glm::Vec3>(
        shader: Shader,
        primitive: Primitive,
        colour_func: F,
    ) -> Object {
        let mut vertices = Vec::new();
        for p in &primitive.points {
            vertices.push(Vertex::new(p, &colour_func(p)));
        }
        Object {
            shader: shader,
            primitive: primitive,
            vertices: vertices,
        }
    }

    pub fn update<F: Fn(&glm::Vec3) -> glm::Vec3>(&mut self, point_func: F, colour_func: F) {
        self.primitive.update(point_func);
        for (v, p) in self.vertices.iter_mut().zip(self.primitive.points.iter()) {
            v.update(p, &colour_func(p));
        }
    }

    pub fn make_buffers(&self, display: &glium::Display) -> Buffers {
        Buffers {
            vertex_buffer: glium::VertexBuffer::new(display, &self.vertices).unwrap(),
            index_buffer: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &self.primitive.indices,
            ).unwrap(),
        }
    }
}
