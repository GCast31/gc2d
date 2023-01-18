
use std::time::{Instant, Duration};

use crate::{window::Window, graphics::{Graphics, self}, event::{Event, EventLoop, EventError}, context::Context, fonts::{FontsManager, FontContext}};


pub struct Gc2d<'a> {
   pub window: Window,
   pub graphics: Graphics,
   pub event: Event,
   _context: Context,
   pub fonts: FontsManager<'a, 'a>,

   pub max_fps: u32,
}

impl Gc2d<'_> {
    pub fn new() -> Self {
        let context: Context = Context::new();
        let event: Event = Event::new(&context);
        let window = Window::new();
        let mut graphics: Graphics = Graphics::new(&context, &window);
        let fc = graphics.get_fonts_creator();
        let fonts = FontsManager::new(fc);

        Self {
            window,
            _context: context,
            event,
            graphics,
            fonts,
            max_fps: 60,
        }
    }

    pub fn create_fonts_context() -> FontContext<'static> {
        sdl2::ttf::init().unwrap() as FontContext
    }
    
    pub fn run(&mut self, mut game: impl EventLoop) -> Result<(), EventError>{

        game.load(self)?;

        self.window.update(&mut self.graphics);
        
        let mut timer_start: Instant = Instant::now();

        // Main loop
        'mainloop: loop {
            // Before drawing
            self.graphics.begin_draw();

            // Keys
            for event in self.event.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => {
                        break 'mainloop;
                    },
                    _ => {},
                }
            }

            // Update
            let dt: f32 = timer_start.elapsed().as_secs_f32();
            timer_start = Instant::now();
            game.update(self, dt)?;

            // Drawing
            game.draw(self)?;

            // End
            self.graphics.end_draw();
            
            // Limit FPS
            if self.max_fps > 0 {
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.max_fps));
            }
        }

        Ok(())
    }

    pub fn set_max_fps(&mut self, fps: u32) {
        self.max_fps = fps;
    }

}
