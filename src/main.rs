
use gc2d::color::Color;
use gc2d::gc2d::Gc2d;
use gc2d::event::EventLoop;

struct MyTestApp {

}

impl EventLoop for MyTestApp {
    fn load(&mut self, gc2d: &mut Gc2d) -> Result<(), gc2d::event::EventError> {
        Ok(())
    }

    fn update(&mut self, gc2d: &mut Gc2d, dt: f32) -> Result<(), gc2d::event::EventError> {
        if gc2d.keyboard.is_down(gc2d::keyboard::KeyCode::A) {
            println!("A is pressed");
        }
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut Gc2d, fonts: &mut gc2d::fonts::FontsManager) -> Result<(), gc2d::event::EventError> {
        gc2d.graphics.circle(gc2d::graphics::DrawMode::Line, 50f32, 50f32, 20f32, Some(Color::BLUE));
        Ok(())
    }

}

fn main() {
    Gc2d::new()
        .run(MyTestApp {

        }).unwrap();
}