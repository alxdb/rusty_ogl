use glium::{self, glutin, Surface};
use glm;

mod geometry;
use self::geometry::{Buffers, Object};

pub struct World {
    events_loop: glutin::EventsLoop,
    display: glium::Display,
    objects: Vec<(Object, Buffers)>,
}

impl World {
    pub fn draw(&self, objects: Vec<Object>) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.015, 0.015, 0.015, 1.0), 1.0);

        let window_dims = self.display.get_framebuffer_dimensions();
        let screen_ratio = window_dims.0 as f32 / window_dims.1 as f32;
        let projection = glm::perspective(::std::f32::consts::FRAC_PI_2, screen_ratio, 0.01, 100.0);

        let view = glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, -1.0));

        for object in objects {
            object.draw(self.display).unwrap();
        }
    }
}

pub struct WorldBuilder {
    pub fullscreen: bool,
    pub msaa: bool,
}

impl Default for WorldBuilder {
    fn default() -> WorldBuilder {
        WorldBuilder {
            fullscreen: true,
            msaa: true,
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

        World {
            events_loop: events_loop,
            display: display,
        }
    }
}
