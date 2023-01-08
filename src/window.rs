use sdl2::video::FullscreenType;

use crate::graphics::Graphics;

pub struct Window {   
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) title: String,
    pub(crate) fullscreen: bool,
}

impl Window {

    pub(crate) fn new() -> Self {
        Self {
            width: 800.,
            height: 600.,
            title: String::from(""),
            fullscreen: false,
        }
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn update(&self, graphics: &mut Graphics) {
        let window = graphics.canvas.window_mut();
        window.set_title(self.title.as_str()).unwrap();
        window.set_size(self.width as u32, self.height as u32).unwrap();
        window.set_fullscreen(
            if self.fullscreen == false { 
                sdl2::video::FullscreenType::Off 
            } else  {
                FullscreenType::True
            }).unwrap();
    }

}