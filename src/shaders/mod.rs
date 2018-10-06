use glium;

pub struct Shader {
    pub vertex_source: &'static str,
    pub fragment_source: &'static str,
}

impl Shader {
    pub fn make_program(&self, display: &glium::Display) -> glium::Program {
        glium::Program::from_source(display, self.vertex_source, self.fragment_source, None)
            .unwrap()
    }
}

impl Default for Shader {
    fn default() -> Self {
        Shader {
            vertex_source: include_str!("basic.vs.glsl"),
            fragment_source: include_str!("basic.fs.glsl"),
        }
    }
}

// pub const BASIC: Shader = Shader {
//     vertex_source: include_str!("basic.vs.glsl"),
// };
