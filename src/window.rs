

pub struct Window {
    window_mod: WindowMod,
}

impl Default for Window {
    fn default() -> Self {
        Self { 
            window_mod: WindowMod::default(),
        }
    }
}

impl Window {
    pub fn get_width(&self) -> f32 {
        self.window_mod.width
    }
    pub fn get_height(&self) -> f32 {
        self.window_mod.height
    }
}

pub struct WindowMod {
    pub width: f32,
    pub height: f32,
    pub fullscreen: bool,
    pub resizable: bool,
    pub centered: bool,
}

impl Default for WindowMod {
    fn default() -> Self {
        Self { 
            width: 800., 
            height: 600., 
            fullscreen: false, 
            resizable: false, 
            centered: false, 
        }
    }
}