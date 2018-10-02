#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;
extern crate rusty_ogl;

use glium::glutin;
use glium::Surface;
use rusty_ogl::make_square;

fn main() {
    let (display, mut events_loop) = init();

    let square = make_square(glm::vec3(-0.5, -0.5, 0.0), 1.0);
    let vertex_buffer = glium::VertexBuffer::new(&display, &square.0).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &square.1,
    ).unwrap();

    let program = glium::Program::from_source(
        &display,
        include_str!("shaders/basic.vs.glsl"),
        include_str!("shaders/basic.fs.glsl"),
        None,
    ).unwrap();

    let inc: f32 = 0.001;
    let max: f32 = 0.5;
    let min: f32 = -0.5;
    let mut dir: i32 = 1;
    let mut t = min;
    main_loop(&mut events_loop, || {
        let mut target = display.draw();
        target.clear_color(0.015, 0.015, 0.015, 1.0);

        t += (dir as f32) * inc;
        if t > max || t < min {
            dir *= -1;
        };

        let transform = glm::rotate(
            &glm::translate(&glm::identity(), &glm::vec3(t, 0.0, 0.0)),
            t * std::f32::consts::PI,
            &glm::vec3(0.0, 0.0, 1.0),
        );

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform! {transform: *transform.as_ref()},
                &Default::default(),
            ).unwrap();

        target.finish().unwrap();
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
    let context = glutin::ContextBuilder::new();
    (
        glium::Display::new(window, context, &events_loop).unwrap(),
        events_loop,
    )
}
