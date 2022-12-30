use sdl2::Sdl;



pub struct Context {
    pub context: Sdl,
}

impl Context {
    pub fn new() -> Self {
        Self {
            context: sdl2::init().unwrap(),
        }
    }
}