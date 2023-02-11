use std::collections::HashMap;

use sdl2::EventPump;

pub struct MouseButton {
    pressed: bool,
    released: bool,
    old_pressed: bool,
    just_pressed: bool,
    just_released: bool,
}

impl Default for MouseButton {
    fn default() -> Self {
        Self {
            old_pressed: false,
            pressed: false,
            released: false,
            just_pressed: false,
            just_released: false,
        }
    }
}

impl MouseButton {
    fn set_state(&mut self, pressed: bool) {
        self.old_pressed = self.pressed;
        self.pressed = pressed;
        self.released = !pressed;
        self.just_pressed = !self.old_pressed && self.pressed;
        self.just_released = self.old_pressed && self.released;
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum MouseButtonId {
    Right,
    Left,
}

pub struct Mouse {
    pub x: f32,
    pub y: f32,
    pub buttons: HashMap<MouseButtonId, MouseButton>,
}

impl Mouse {
    pub fn new() -> Self {
        Self {  
            x: 0.,
            y: 0.,
            buttons: HashMap::from([
                (MouseButtonId::Right, MouseButton::default()),
                (MouseButtonId::Left, MouseButton::default())
            ]),
        }
    }

    pub(crate) fn update(&mut self, event_pump: &EventPump) {

        let state = event_pump.mouse_state();
        
        self.x = state.x() as f32;
        self.y = state.y() as f32;

        for (id, button) in self.buttons.iter_mut() {
            match id {
                MouseButtonId::Left => {
                    button.set_state(state.left());
                },
                MouseButtonId::Right => {
                    button.set_state(state.right());
                },
            }
        }
    }

    pub fn button_is_pressed(&self, id: MouseButtonId) -> bool {
        if let Some(button) = self.buttons.get(&id) {
          button.pressed
        } else {
            false
        } 
    }

    pub fn button_is_released(&self, id: MouseButtonId) -> bool {
        if let Some(button) = self.buttons.get(&id) {
          button.released
        } else {
            false
        } 
    }

    pub fn button_is_just_released(&self, id: MouseButtonId) -> bool {
        if let Some(button) = self.buttons.get(&id) {
          button.just_released
        } else {
            false
        } 
    }

    pub fn button_is_just_pressed(&self, id: MouseButtonId) -> bool {
        if let Some(button) = self.buttons.get(&id) {
          button.just_pressed
        } else {
            false
        } 
    }
}