use gc2d::{self, event::EventLoop, event::Event, gc2d::Gc2d, color::Color};

//------------------------------------------------------------------
//                                 MAIN
//------------------------------------------------------------------
fn main() {
    let gc2d = Gc2d::new();

    let game = Game {

    };

    Event::run(game, gc2d).unwrap();
}


//------------------------------------------------------------------
//                                 EXAMPLE
//------------------------------------------------------------------
struct Game {

}

impl EventLoop for Game {
    fn load<'a>(&mut self, gc2d: &'a qmut Gc2d) -> Result<(), gc2d::event::EventError> {

        gc2d.window.set_title("My Title");
        gc2d.window.set_size(1024., 980.);

        gc2d.graphics.new_font(String::from("fonts/Vera.ttf"), 11 as u16);

        Ok(())
    }

    fn draw(&mut self, gc2d: &mut Gc2d) -> Result<(), gc2d::event::EventError> {
        gc2d.graphics.set_color(Color::BLUE);
        gc2d.graphics.line(10., 100., 150., 100., None);
        Ok(())
    }

    fn update(&mut self, gc2d: &mut Gc2d, dt: f32) -> Result<(), gc2d::event::EventError> {
        
        Ok(())
    }
}
