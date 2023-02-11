
use gc2d::color::Color;
use gc2d::gc2d::Gc2d;
use gc2d::event::EventLoop;

struct MyTestApp {
    x: f32,
}

impl EventLoop for MyTestApp {
    fn load(&mut self, gc2d: &mut Gc2d, audio_manager: &mut gc2d::audio::AudioManager) -> Result<(), gc2d::event::EventError> {
        //gc2d.audio.new_source("assets/sounds/cool.mp3", audio_manager, gc2d::audio::AudioType::Stream);
        //gc2d.audio.play(audio_manager, "assets/sounds/cool.mp3", -1);
        
        gc2d.graphics.new_font("assets/fonts/PixelMaster.ttf", 25);
        Ok(())
    }

    fn update(&mut self, gc2d: &mut Gc2d, dt: f32, audio_manager: &mut gc2d::audio::AudioManager) -> Result<(), gc2d::event::EventError> {
        self.x = gc2d.mouse.x;
        
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut Gc2d, fonts: &mut gc2d::fonts::FontsManager, dt: f32) -> Result<(), gc2d::event::EventError> {
        gc2d.graphics.circle(gc2d::graphics::DrawMode::Line, self.x, 50f32, 20f32, Some(Color::BLUE));
        Ok(())
    }

}

fn main() {
    Gc2d::new()
        .run(MyTestApp {
            x: 50f32,
        }).unwrap();
}