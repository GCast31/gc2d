use std::time::Instant;

use sdl2::EventPump;

use crate::{gc2d::Gc2d, context::Context};


pub enum EventError {

}

pub trait GameLoop {
    fn load(&mut self) -> Result<(), EventError> {
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut Gc2d) -> Result<(), EventError> {
        Ok(())
    }

    fn update(&mut self, gc2d: &mut Gc2d, dt: f32) -> Result<(), EventError> {
        Ok(())
    }
}

pub struct Event {
    pub event_pump: EventPump,
}

impl Event {
    pub fn new(ctx: &Context) -> Self {
        Self {
            event_pump: ctx.context.event_pump().unwrap(), 
        }
    }

    pub fn run(mut game: impl GameLoop, mut gc2d: Gc2d) -> Result<(), EventError>{

        game.load()?;
        
        let mut timer_start: Instant = Instant::now();

        // Main loop
        'mainloop: loop {
            // Before drawing
            gc2d.graphics.begin_draw();

            // Keys
            for event in gc2d.event.event_pump.poll_iter() {
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
            game.update(&mut gc2d, dt)?;
        }

        Ok(())
    }
}