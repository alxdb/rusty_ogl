use glium::{self, glutin, Surface};
use std::thread::sleep;
use std::time::{Duration, Instant};

// use geometry::Object;
// use geometry::Shape;

pub struct World {
    events_loop: glutin::EventsLoop,
    display: glium::Display,
    spf: Duration,
    frames: u32,
}

impl World {
    pub fn draw_loop<F: FnMut(&mut glium::Frame, (u32, u32), u32)>(&mut self, mut draw_func: F) {
        let mut closed = false;
        while !closed {
            let start = Instant::now();

            let mut target = self.display.draw();
            target.clear_color_and_depth((0.015, 0.015, 0.015, 1.0), 1.0);

            draw_func(
                &mut target,
                self.display.get_framebuffer_dimensions(),
                self.frames,
            );

            target.finish().unwrap();

            self.events_loop.poll_events(|ev| match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode
                    {
                        Some(glutin::VirtualKeyCode::Escape) => closed = true,
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            });

            let end = Instant::now();
            let duration = end - start;
            if duration < self.spf {
                sleep(self.spf - duration);
            }

            self.frames += 1;
        }
    }

    pub fn get_display(&self) -> &glium::Display {
        &self.display
    }
}

pub struct WorldBuilder {
    pub fullscreen: bool,
    pub msaa: bool,
    pub fps: f32,
}

impl Default for WorldBuilder {
    fn default() -> WorldBuilder {
        WorldBuilder {
            fullscreen: true,
            msaa: true,
            fps: 30.0,
        }
    }
}

impl WorldBuilder {
    pub fn build(&self) -> World {
        let events_loop = glutin::EventsLoop::new();

        let window = if self.fullscreen {
            glutin::WindowBuilder::new().with_fullscreen(Some(events_loop.get_primary_monitor()))
        } else {
            glutin::WindowBuilder::new()
        };

        let context = if self.msaa {
            glutin::ContextBuilder::new()
                .with_multisampling(8)
                .with_depth_buffer(24)
        } else {
            glutin::ContextBuilder::new()
        };

        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let spf = Duration::from_nanos(((1.0 / self.fps) * 1e9) as u64);
        World {
            events_loop: events_loop,
            display: display,
            spf: spf,
            frames: 0,
        }
    }
}
