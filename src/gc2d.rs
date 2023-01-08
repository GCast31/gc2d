
use crate::{window::Window, graphics::Graphics, event::Event, context::Context, fonts::FontsManager};


pub struct Gc2d<'a> {
   pub window: Window,
   pub graphics: Graphics<'a>,
   pub event: Event,
   _context: Context,
}

impl<'a> Gc2d<'a> {
    pub fn new() -> Self {
        let context: Context = Context::new();
        let event: Event = Event::new(&context);
        let window = Window::new();
        let graphics: Graphics = Graphics::new(&context, &window);

        Self {
            window,
            _context: context,
            event,
            graphics,
        }
    }
}
