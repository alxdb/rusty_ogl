use glium::{self, index::PrimitiveType, uniforms};
use glm;

mod primitives;
pub use self::primitives::Shape;
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
            position: (*position).into(),
            colour: (*colour).into(),
        }
    }
}

struct Buffers {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<Index>,
}

impl Buffers {
    fn new(
        display: &glium::Display,
        vertices: &[Vertex],
        indices: &(PrimitiveType, Vec<Index>),
    ) -> Buffers {
        Buffers {
            vertex_buffer: glium::VertexBuffer::dynamic(display, vertices).unwrap(),
            index_buffer: glium::IndexBuffer::persistent(display, indices.0, &indices.1).unwrap(),
        }
    }
}

pub struct Object {
    init_vertices: Vec<Vertex>,
    vertices: Vec<Vertex>,
    buffers: Buffers,
    pub global_transform: glm::Mat4,
}

impl Object {
    pub fn new<F: Fn(&glm::Vec3) -> glm::Vec3>(
        display: &glium::Display,
        shape: Shape,
        colour_func: F,
    ) -> Object {
        let mut vertices = Vec::new();
        for p in &shape.points {
            vertices.push(Vertex::new(p, &colour_func(p)));
        }
        let buffers = Buffers::new(display, &vertices, &shape.indices);
        Object {
            init_vertices: vertices.clone(),
            vertices,
            buffers,
            global_transform: glm::identity(),
        }
    }

    pub fn transform<F: Fn(glm::Vec3, glm::Vec3) -> (glm::Vec3, glm::Vec3)>(&mut self, f: F) {
        for (v, i_v) in self.vertices.iter_mut().zip(self.init_vertices.iter()) {
            let (pos, col) = f(
                glm::Vec3::from_row_slice(&i_v.position),
                glm::Vec3::from_row_slice(&i_v.colour),
            );
            v.position = pos.into();
            v.colour = col.into();
        }
        self.buffers.vertex_buffer.write(&self.vertices);
    }

    pub fn reinit(&mut self) {
        self.init_vertices = self.vertices.clone();
    }

    pub fn draw(
        &self,
        surface: &mut impl glium::Surface,
        program: &glium::Program,
        uniforms: &impl uniforms::Uniforms,
        draw_parameters: &glium::DrawParameters,
    ) -> Result<(), glium::DrawError> {
        surface.draw(
            &self.buffers.vertex_buffer,
            &self.buffers.index_buffer,
            program,
            uniforms,
            draw_parameters,
        )
    }
}
