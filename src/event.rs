
use sdl2::EventPump;

use crate::{gc2d::{Gc2d, Gc2dResult}, context::Context, fonts::FontsManager, keyboard::KeyCode, audio::AudioManager};

#[allow(unused_variables)]
pub trait EventLoop {
    fn load(&mut self, gc2d: &mut Gc2d, audio_manager: &mut AudioManager) -> Gc2dResult<()> {
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut Gc2d, fonts: &mut FontsManager, dt: f32) -> Gc2dResult<()> {
        Ok(())
    }

    fn update(&mut self, gc2d: &mut Gc2d, dt: f32, audio_manager: &mut AudioManager) -> Gc2dResult<()> {
        Ok(())
    }

    fn key_pressed(&mut self, gc2d: &mut Gc2d, key: KeyCode, audio_manager: &mut AudioManager) -> Gc2dResult<()> {
        Ok(())
    }

    fn key_released(&mut self, gc2d: &mut Gc2d, key: KeyCode, audio_manager: &mut AudioManager) -> Gc2dResult<()> {
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
}