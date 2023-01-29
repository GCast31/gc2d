
use gc2d::color::Color;
use gc2d::gc2d::Gc2d;
use gc2d::event::EventLoop;

struct MyTestApp {

}

impl EventLoop for MyTestApp {
    fn load(&mut self, gc2d: &mut Gc2d) -> Result<(), gc2d::event::EventError> {
        gc2d.window.set_title("MyApp");
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut Gc2d, fonts: &mut gc2d::fonts::FontsManager) -> Result<(), gc2d::event::EventError> {
        Ok(())
    }
}

fn main() {
    Gc2d::new()
        .run(MyTestApp {

        }).unwrap();
}