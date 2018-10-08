#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;
extern crate rusty_ogl;

use rusty_ogl::geometry;
use rusty_ogl::shaders;
use rusty_ogl::world;

use std::f32::consts;

fn main() {
    let mut world = world::WorldBuilder {
        ..Default::default()
    }.build();

    let draw_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut plane = geometry::Object::new(
        world.get_display(),
        geometry::Shape::plane(&glm::vec3(-1.0, -1.0, 0.0), 2.0, 100),
        |p| {
            let dist = glm::distance(p, &glm::Vec3::zeros());
            glm::Vec3::repeat(dist)
        },
    );

    // plane.transform(|p, c| {
    //     let polar = p.y * std::f32::consts::PI;
    //     let azimuth = p.x * std::f32::consts::FRAC_PI_2;
    //     (
    //         glm::vec3(
    //         polar.sin() * azimuth.cos(),
    //         polar.sin() * azimuth.sin(),
    //         polar.cos())
    //     , glm::vec3((p.x + 1.0) / 2.0, (p.y + 1.0) / 2.0, 0.0))
    // })

    let mut sphere = geometry::Object::new(
        world.get_display(),
        geometry::Shape::sphere(&glm::Vec3::zeros(), 1.0, 50),
        |p| {
            let dist = glm::distance(&glm::vec3(0.0, 1.0, 0.0), p);
            glm::Vec3::repeat(dist * 0.25)
        },
    );

    let mut cube = geometry::Object::new(
        world.get_display(),
        geometry::Shape::cube(&glm::vec3(-0.5, -0.5, -0.5), 1.0),
        |_| glm::vec3(0.1, 0.25, 0.6),
    );

    let program = shaders::Shader {
        ..Default::default()
    }.make_program(world.get_display());

    world.draw_loop(|surface, dims, frame| {
        let aspect = dims.0 as f32 / dims.1 as f32;
        let time = frame as f32;

        let zoom = 0.5;
        // let projection: glm::Mat4 = glm::ortho(-aspect / zoom, aspect / zoom, -1.0 / zoom, 1.0 / zoom, 0.0, 100.0);
        let projection: glm::Mat4 = glm::perspective(consts::FRAC_PI_2, aspect, 0.01, 100.0);

        let view: glm::Mat4 = glm::look_at(&glm::vec3(0.0, 1.0, 3.0), &glm::vec3(0.0, 0.0, 0.0), &glm::vec3(0.0, 1.0, 0.0));

        let uniform = &uniform!{ projection: *projection.as_ref(), view: *view.as_ref(), transform: *sphere.global_transform.as_ref() };
        sphere.draw(surface, &program, uniform, &draw_params).unwrap();
        sphere.global_transform = glm::rotation(time * 0.05, &glm::Vec3::y_axis());

        sphere.transform(|mut p, c| {
            let dist = glm::distance(&glm::vec3(0.0, 1.0, 0.0), &p);
            let wave = (time * 0.1 + dist * 8.0).sin() * 0.1;
            p += p.normalize() * wave;
            (p, c)
        });

        cube.draw(
            surface,
            &program,
            &uniform!{ projection: *projection.as_ref(), view: *view.as_ref(), transform: *cube.global_transform.as_ref() },
            &draw_params
        ).unwrap();
        cube.global_transform = glm::rotation(time * 0.05, &glm::Vec3::y_axis());

        cube.transform(|p, c| {
            let dist = glm::distance(&p, &glm::Vec3::zeros());
            let wave = (time * 0.01).sin() * dist;
            (p + glm::vec3(0.0, 0.0, wave), c)
        });

        plane.draw(surface, &program, uniform, &draw_params).unwrap();
    });
}
