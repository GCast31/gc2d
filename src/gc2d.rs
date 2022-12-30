
use crate::{window::Window, graphics::Graphics, event::Event, context::Context};


pub struct Gc2d {
   pub window: Window,
   pub graphics: Graphics,
   pub event: Event,
   pub context: Context,
}

impl Gc2d {
    pub fn new() -> Self {
        let window = Window::default();
        let context: Context = Context::new();
        let event: Event = Event::new(&context);
        let graphics: Graphics = Graphics::new(&window, &context);
        Self {
            window,
            context,
            event,
            graphics,
        }
    }
}
