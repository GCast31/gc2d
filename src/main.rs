use gc2d::{self, event::EventLoop, event::Event, gc2d::Gc2d, color::Color, fonts::{FontsManager, FontContext, FontKey}};

//------------------------------------------------------------------
//                                 MAIN
//------------------------------------------------------------------
fn main() {


    let mut gc2d = Gc2d::new();
    let my_font = gc2d.fonts.new_font("fonts/Vera.ttf", 30).unwrap();
    //gc2d.fonts.new_font(&mut fonts_context, String::from("coucou"), 10);

    //let fonts = FontsManager::new(gc2d., context)


    // Game
    let game = Game {
    };

    // Run
    gc2d.run(game);
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

        let fontkey = FontKey {
            filename: String::from("fonts/Vera.ttf"),
            point_size: 10,
        };

        gc2d.graphics.set_font(Some(fontkey));

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
