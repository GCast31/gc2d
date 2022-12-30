use gc2d::{self, event::GameLoop, event::Event};

struct Game {

}

impl GameLoop for Game {
    fn load(&mut self) -> Result<(), gc2d::event::EventError> {
        println!("coucou");
        Ok(())
    }
}

fn main() {
    let gc2d = gc2d::gc2d::Gc2d::new();
    let game = Game {

    };

    Event::run(game, gc2d);
}
