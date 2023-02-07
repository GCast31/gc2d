
use sdl2::EventPump;

use crate::{gc2d::Gc2d, context::Context, fonts::{FontsManager}, keyboard::KeyCode};


#[derive(Debug)]
pub enum EventError {

}

pub trait EventLoop {
    fn load(&mut self, gc2d: &mut Gc2d) -> Result<(), EventError> {
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut Gc2d, fonts: &mut FontsManager) -> Result<(), EventError> {
        Ok(())
    }

    fn update(&mut self, gc2d: &mut Gc2d, dt: f32) -> Result<(), EventError> {
        Ok(())
    }

    fn key_pressed(&mut self, gc2d: &mut Gc2d, key: KeyCode) -> Result<(), EventError> {
        Ok(())
    }

    fn key_released(&mut self, gc2d: &mut Gc2d, key: KeyCode) -> Result<(), EventError> {
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