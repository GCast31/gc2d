use std::time::{Instant, Duration};

use gc2d::{self, event::EventLoop, event::Event, gc2d::Gc2d, color::Color, fonts::{FontsManager, Font}};

//------------------------------------------------------------------
//                                 MAIN
//------------------------------------------------------------------
fn main() {

    Gc2d::new().run(
        Game {

        })
        .unwrap();

}


//------------------------------------------------------------------
//                                 EXAMPLE
//------------------------------------------------------------------
struct Game {
}

impl EventLoop for Game {
    fn load(&mut self, gc2d: &mut Gc2d) -> Result<(), gc2d::event::EventError> {

        gc2d.window.set_title("My Title");
        gc2d.window.set_size(1024., 980.);

        gc2d.graphics.new_font("fonts/Vera.ttf", 20);
        gc2d.graphics.set_font(Some(Font { filename: String::from("fonts/Vera.ttf"), point_size: 20 }));

        Ok(())
    }

    fn draw(&mut self, gc2d: &mut Gc2d, fonts: &mut FontsManager) -> Result<(), gc2d::event::EventError> {
        gc2d.graphics.set_color(Color::BLUE);
        gc2d.graphics.line(10., 100., 150., 100., None);

        gc2d.graphics.print(fonts, String::from("Coucou"), 10f32, 10f32, Some(Color::BLUE));

        Ok(())
    }

    fn update(&mut self, gc2d: &mut Gc2d, dt: f32) -> Result<(), gc2d::event::EventError> {
        
        Ok(())
    }
}
