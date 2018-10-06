use glm;

pub type Index = u16;

#[derive(Clone)]
pub struct Primitive {
    pub points: Vec<glm::Vec3>,
    pub indices: Vec<Index>,
}

impl Primitive {
    // Constructors
    pub fn plane(origin: &glm::Vec3, size: f32, divisions: u16) -> Self {
        let spacing = size / divisions as f32;
        let mut points = Vec::new();
        let mut indices = Vec::new();
        for i in 0..=divisions {
            for j in 0..=divisions {
                points.push(origin + glm::vec3(spacing * j as f32, spacing * i as f32, 0.0));
                if j < divisions && i < divisions {
                    let index = i * (divisions + 1) + j;
                    indices.append(&mut vec![
                        index,
                        index + 1,
                        index + divisions + 1,
                        index + 1,
                        index + divisions + 1,
                        index + divisions + 2,
                    ]);
                }
            }
        }
        Primitive {
            points: points,
            indices: indices,
        }
    }

    pub fn triangle(a: &glm::Vec3, b: &glm::Vec3, c: &glm::Vec3) -> Self {
        Primitive {
            points: vec![a.clone(), b.clone(), c.clone()],
            indices: vec![0, 1, 2],
        }
    }

    pub fn square(origin: &glm::Vec3, size: f32) -> Self {
        Primitive {
            points: vec![
                origin.clone(),
                origin + glm::vec3(size, 0.0, 0.0),
                origin + glm::vec3(0.0, size, 0.0),
                origin + glm::vec3(size, size, 0.0),
            ],
            indices: vec![0, 1, 2, 1, 2, 3],
        }
    }

    // Methods
    pub fn update<F: Fn(&glm::Vec3) -> glm::Vec3>(&mut self, f: F) {
        for p in &mut self.points {
            *p = f(p);
        }
    }

    pub fn transform<F: Fn(&glm::Vec3) -> glm::Vec3>(&self, f: F) -> Self {
        let mut new_prim = self.clone();
        new_prim.update(f);
        new_prim
    }
}
