
/*
 * For using gc2d
 * ==============
 * 
 * Create a struct  
 * Implement EventLoop for the struct
 * 
 * Gc2d::new().run(the struct);
 * 
 * Todo : Audio multi-chanel
 * Todo : Audio Stop / Pause ...
 * Todo : FPS to optimize
 * Todo : Optimize graphics draw
 * Todo : Add more bindings keys in keyboard
 * Todo : Add error
 * Todo : Test Quad / scale ...
 */

use gc2d::color::Color;
use gc2d::gc2d::{Gc2d, Gc2dResult};
use gc2d::event::EventLoop;

struct MyTestApp {
    x: f32,
}

impl EventLoop for MyTestApp {
    fn load(&mut self, gc2d: &mut Gc2d, _audio_manager: &mut gc2d::audio::AudioManager) -> Gc2dResult<()> {
        //gc2d.audio.new_source("assets/sounds/cool.mp3", audio_manager, gc2d::audio::AudioType::Stream);
        //gc2d.audio.play(audio_manager, "assets/sounds/cool.mp3", -1);
        gc2d.graphics.set_background_color(Color::WHITE);
        gc2d.graphics.new_font("assets/fonts/PixelMaster.ttf", 18);
        Ok(())
    }

    fn update(&mut self, gc2d: &mut Gc2d, _dt: f32, _audio_manager: &mut gc2d::audio::AudioManager) -> Gc2dResult<()> {
        self.x = gc2d.mouse.x;
        
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut Gc2d, fonts: &mut gc2d::fonts::FontsManager, _dt: f32) -> Gc2dResult<()> {
        gc2d.graphics.circle(gc2d::graphics::DrawMode::Line, self.x, 50f32, 20f32, Some(Color::BLUE));
        gc2d.graphics.print(format!("x: {}", gc2d.mouse.x), 10., 10., Some(Color::RED), fonts);
        gc2d.graphics.print(format!("Hello, world [width : {}]", gc2d.graphics.get_text_width("Hello, world", fonts, String::from("assets/fonts/PixelMaster.ttf"), 18).unwrap()), 10., 20., Some(Color::BLUE), fonts);
        Ok(())
    }

}

fn main() {
    Gc2d::new()
        .run(MyTestApp {
            x: 50f32,
        }).unwrap();
}