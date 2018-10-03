#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;
extern crate rusty_ogl;

use glium::glutin;
use glium::Surface;
use rusty_ogl::Colour;
use rusty_ogl::Object;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let (display, mut events_loop) = init();

    // let square = Object::square(&glm::vec3(-0.5, -0.5, 0.0), 1.0);
    let square = Object::plane(&glm::vec3(-0.5, -0.5, 0.0), 1.0, 50);
    // let triangle = Object::triangle(
    //     &glm::vec3(-0.5, -0.5, 0.0),
    //     &glm::vec3(0.5, -0.5, 0.0),
    //     &glm::vec3(0.0, 0.5, 0.0),
    // );

    let program = glium::Program::from_source(
        &display,
        include_str!("shaders/basic.vs.glsl"),
        include_str!("shaders/basic.fs.glsl"),
        None,
    ).unwrap();

    let draw_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let spf = Duration::from_nanos(((1.0 / 30.0) * 1e9) as u64);
    let mut frame_count = 0;
    main_loop(&mut events_loop, || {
        let start = Instant::now();
        let time = frame_count as f32;
        let mut target = display.draw();
        target.clear_color_and_depth((0.015, 0.015, 0.015, 1.0), 1.0);

        let window_dims = display.get_framebuffer_dimensions();
        let screen_ratio = window_dims.0 as f32 / window_dims.1 as f32;
        let perspective: glm::Mat4 = glm::ortho(-screen_ratio, screen_ratio, -1.0, 1.0, 0.0, 100.0);
        // let perspective: glm::Mat4 =
        //     glm::perspective(std::f32::consts::FRAC_PI_2, screen_ratio, 0.0, -100.0);
        let mut transform: glm::Mat4 = glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, -1.0));
        // let mut transform: glm::Mat4 = glm::identity();

        transform = glm::rotate(
            &transform,
            std::f32::consts::FRAC_PI_3,
            &glm::vec3(-1.0, 0.0, 0.0),
        );
        transform = glm::rotate(
            &transform,
            std::f32::consts::FRAC_PI_4,
            &glm::vec3(0.0, 0.0, 1.0),
        );

        let r_square = square.transform(|v| {
            let pos = v.pos();
            let origin = glm::vec3(0.0, 0.0, 0.0);
            let phase = glm::distance(&pos, &origin) * 20.0;
            let wave = (time * 0.05 + phase).sin() * 0.1;
            let wave2 = (-time * 0.05 - phase).sin() * 0.1;
            let mut vertex = rusty_ogl::Vertex::new(&(pos + glm::vec3(0.0, 0.0, wave)));
            // println!("{}", v.pos().transpose());
            vertex.set_colour(&Colour(0.0, (wave2 * 2.5) + 0.5, (wave * 2.5) + 0.5));
            vertex
        });

        target
            .draw(
                &r_square.vertex_buffer(&display),
                &r_square.index_buffer(&display),
                &program,
                &uniform! {perspective: *perspective.as_ref(), transform: *transform.as_ref(), frame: frame_count},
                &draw_params,
            ).unwrap();

        target.finish().unwrap();
        let end = Instant::now();
        let duration = end - start;
        if duration < spf {
            sleep(spf - duration);
        }
        let frame_dur = Instant::now() - start;
        let frame_time = frame_dur.as_secs() as f64 + (frame_dur.subsec_nanos() as f64 * 1e-9);
        if frame_count % 10 == 0 {
            println!("fps: {}", 1.0 / frame_time);
        }
        frame_count += 1;
    });
}

fn main_loop<F: FnMut()>(events_loop: &mut glutin::EventsLoop, mut f: F) {
    let mut closed = false;
    while !closed {
        f();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(glutin::VirtualKeyCode::Escape) => closed = true,
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        });
    }
}

fn init() -> (glium::Display, glutin::EventsLoop) {
    let events_loop = glutin::EventsLoop::new();
    let window =
        glutin::WindowBuilder::new().with_fullscreen(Some(events_loop.get_primary_monitor()));
    // glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_multisampling(8)
        .with_depth_buffer(24);
    (
        glium::Display::new(window, context, &events_loop).unwrap(),
        events_loop,
    )
}
