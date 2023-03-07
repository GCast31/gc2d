
use std::time::{Instant, Duration};
use crate::{window::Window, graphics::Graphics, event::{Event, EventLoop}, context::Context, fonts::FontsManager, keyboard::Keyboard, audio::AudioManager, mouse::Mouse};
use crate::audio::Audio;

pub type Gc2dResult<T> = Result<T, String>;

pub struct Gc2d {
   _context: Context,

   pub window: Window,
   pub graphics: Graphics,
   pub event: Event,
   pub keyboard: Keyboard,
   pub audio: Audio,
   pub mouse: Mouse,

   max_fps: u32,
}

impl Gc2d {
    pub fn new() -> Self {
        
        let context: Context = Context::new();
        let event: Event = Event::new(&context);
        let window = Window::new();
        let graphics: Graphics = Graphics::new(&context, &window);
        let keyboard: Keyboard = Keyboard::new();
        let audio: Audio = Audio::new(&context);
        let mouse = Mouse::new();

        Self {
            _context: context,
            window,
            event,
            graphics,
            max_fps: 60,
            keyboard,
            audio,
            mouse,
        }
    }

    pub fn set_max_fps(&mut self, fps: u32) {
        self.max_fps = fps;
    }


    pub fn run(&mut self, mut game: impl EventLoop) -> Gc2dResult<()> {

        // Initialize font context
        let ttf_context = sdl2::ttf::init().unwrap();

        // Create fonts manager
        let mut fonts_manager = FontsManager::new();

        // Create audio manager
        let mut audio_manager: AudioManager = AudioManager::new();
    
        game.load(self, &mut audio_manager)?;
    
        self.window.update(&mut self.graphics);
        
        let mut timer_start: Instant = Instant::now();

    
        // Main loop
        'mainloop: loop {

            let mut font_clone = None;

            // Add new fonts ?
            for font in self.graphics._new_fonts.iter() {
                fonts_manager.new_font(&ttf_context, font.clone()).unwrap();
                if self.graphics.actual_font.is_none() && font_clone.is_none() {
                    font_clone = Some(font.clone());
                }
            }
            self.graphics._new_fonts.clear();

            if self.graphics.actual_font.is_none() && font_clone.is_some() {
                self.graphics.set_font(font_clone);
            }

            // Before drawing
            self.graphics.begin_draw();
    
            // Update keyboard
            self.keyboard.update(&self.event.event_pump);

            // Update mouse
            self.mouse.update(&self.event.event_pump);

            // Keys
            for event in self.event.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => {
                        break 'mainloop;
                    },
                    _ => {},
                }
            }

            // Key Just pressed
            for key in self.keyboard.get_keys_just_pressed() {
                game.key_pressed(self, key)?;
            }

            // Key Just released
            for key in self.keyboard.get_keys_just_released() {
                game.key_released(self, key)?;
            }
    
            // Update
            let dt: f32 = timer_start.elapsed().as_secs_f32();
            timer_start = Instant::now();
            game.update(self, dt, &mut audio_manager)?;
    
            // Drawing
            game.draw(self, &mut fonts_manager, dt)?;
    
            // End
            self.graphics.end_draw();
            
            // Limit FPS (todo : not optimized ...)
            if self.max_fps > 0 {
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.max_fps));
            }
        }
    
        Ok(())
    }

}

