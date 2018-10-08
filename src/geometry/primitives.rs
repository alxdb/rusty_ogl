use glm;
use std::f32::consts;

pub type Index = u16;

pub struct Shape {
    pub points: Vec<glm::Vec3>,
    pub indices: Vec<Index>,
}

impl Shape {
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
        Shape {
            points: points,
            indices: indices,
        }
    }

    pub fn triangle(a: &glm::Vec3, b: &glm::Vec3, c: &glm::Vec3) -> Self {
        Shape {
            points: vec![a.clone(), b.clone(), c.clone()],
            indices: vec![0, 1, 2],
        }
    }

    pub fn square(origin: &glm::Vec3, size: f32) -> Self {
        Shape {
            points: vec![
                origin.clone(),
                origin + glm::vec3(size, 0.0, 0.0),
                origin + glm::vec3(0.0, size, 0.0),
                origin + glm::vec3(size, size, 0.0),
            ],
            indices: vec![0, 1, 2, 1, 2, 3],
        }
    }

    pub fn cube(origin: &glm::Vec3, size: f32) -> Self {
        let tesselate = |a: Index, b, c, d| vec![a, b, c, b, c, d];
        let mut indices: Vec<Index> = Vec::new();
        indices.append(&mut tesselate(0, 1, 2, 3)); // front
        indices.append(&mut tesselate(2, 3, 6, 7)); // top
        indices.append(&mut tesselate(0, 1, 4, 5)); // bottom
        indices.append(&mut tesselate(4, 5, 6, 7)); // back
        indices.append(&mut tesselate(1, 3, 5, 7)); // right
        indices.append(&mut tesselate(0, 2, 4, 6)); // left
        Shape {
            points: vec![
                origin.clone(),
                origin + glm::vec3(size, 0.0, 0.0),
                origin + glm::vec3(0.0, size, 0.0),
                origin + glm::vec3(size, size, 0.0),
                origin.clone() + glm::vec3(0.0, 0.0, size),
                origin + glm::vec3(size, 0.0, size),
                origin + glm::vec3(0.0, size, size),
                origin + glm::vec3(size, size, size),
            ],
            indices: indices,
        }
    }

    pub fn sphere(origin: &glm::Vec3, radius: f32, divisions: u16) -> Self {
        let mut plane = Shape::plane(&glm::vec3(-1.0, -1.0, 0.0), 2.0, divisions);
        let mut points = Vec::new();
        for p in plane.points.drain(0..) {
            let polar = p.y * consts::PI;
            let azimuth = p.x * consts::FRAC_PI_2;
            let sphere_point = glm::vec3(
                polar.sin() * azimuth.cos(),
                polar.sin() * azimuth.sin(),
                polar.cos(),
            );
            points.push(origin + (sphere_point * radius));
        }
        Shape {
            points: points,
            indices: plane.indices,
        }
    }

    // WIP
    pub fn plane_cube(origin: &glm::Vec3, size: f32, divisions: u16) -> Self {
        let spacing = size / divisions as f32;
        let mut points = Vec::new();
        let on_boundary = |x, y, z| {
            (x == 0 || x == divisions) || (y == 0 || y == divisions) || (z == 0 || z == divisions)
        };
        for x in 0..=divisions {
            for y in 0..=divisions {
                for z in 0..=divisions {
                    if on_boundary(x, y, z) {
                        points.push(glm::vec3(x as f32, y as f32, z as f32) * spacing);
                    }
                }
            }
        }
        Shape {
            points: vec![*origin],
            indices: vec![0],
        }
    }
}
