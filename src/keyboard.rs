use std::collections::HashMap;

use sdl2::{EventPump, keyboard::Keycode};

#[derive(PartialEq, Eq, Clone)]
#[derive(Hash)]
pub enum KeyCode {
    A,
    B,
    D,
    Q,
    S,
    Z,
    Up,
    Down,
    Left,
    Right,
    Space,
}

struct KeyCodeState {
    pressed: bool,
    just_pressed: bool,
    just_released: bool,
    old_pressed: bool,
}

impl KeyCodeState {
    fn new() -> Self {
        Self { 
            pressed: false, 
            just_pressed: false, 
            just_released: false,
            old_pressed: false 
        }
    }
}

pub struct Keyboard {
    keys_states: HashMap<KeyCode, KeyCodeState>,
    matching_table: HashMap<sdl2::keyboard::Keycode, KeyCode>,
}

impl Keyboard {
    pub(crate) fn new() -> Self {

        let mut keys_states = HashMap::new();
        let mut matching_table = HashMap::new();

        Keyboard::add_new_matching(Keycode::A, KeyCode::A, &mut matching_table, &mut keys_states);
        Keyboard::add_new_matching(Keycode::B, KeyCode::B, &mut matching_table, &mut keys_states);
        Keyboard::add_new_matching(Keycode::D, KeyCode::D, &mut matching_table, &mut keys_states);
        Keyboard::add_new_matching(Keycode::Down, KeyCode::Down, &mut matching_table, &mut keys_states);
        Keyboard::add_new_matching(Keycode::Left, KeyCode::Left, &mut matching_table, &mut keys_states);
        Keyboard::add_new_matching(Keycode::Q, KeyCode::Q, &mut matching_table, &mut keys_states);
        Keyboard::add_new_matching(Keycode::Right, KeyCode::Right, &mut matching_table, &mut keys_states);
        Keyboard::add_new_matching(Keycode::S, KeyCode::S, &mut matching_table, &mut keys_states);
        Keyboard::add_new_matching(Keycode::Space, KeyCode::Space, &mut matching_table, &mut keys_states);
        Keyboard::add_new_matching(Keycode::Up, KeyCode::Up, &mut matching_table, &mut keys_states);
        Keyboard::add_new_matching(Keycode::Z, KeyCode::Z, &mut matching_table, &mut keys_states);

        Self { 
            keys_states,
            matching_table,
        }
    }

    fn add_new_matching(from_key: Keycode, to_key: KeyCode, matching_table: &mut HashMap<Keycode, KeyCode>, keys_states: &mut HashMap<KeyCode, KeyCodeState>) {
        keys_states.insert(to_key.clone(), KeyCodeState::new());
        matching_table.insert(from_key, to_key);
    }

    pub(crate) fn update(&mut self, event_pump: &EventPump) {

        for key in self.keys_states.iter_mut() {
            key.1.old_pressed = key.1.pressed;
            key.1.pressed = false;
            key.1.just_pressed = false;
            key.1.just_released = key.1.old_pressed;
        }

        for scancode in event_pump.keyboard_state().pressed_scancodes() {

            let mut matching_key = None;

            if let Some(key_code) = sdl2::keyboard::Keycode::from_scancode(scancode) {
                
                matching_key = self.matching_table.get(&key_code);

            }

            if let Some(key_code) = matching_key {
                if let Some(key_state) = self.keys_states.get_mut(&key_code) {
                    key_state.pressed = true;
                    key_state.just_pressed = !key_state.old_pressed;
                    key_state.just_released = false;
                }
            }
        }

    }

    pub fn is_down(&self, key_code: KeyCode) -> bool {
        if let Some(key_state) = self.keys_states.get(&key_code) {
            return key_state.pressed;
        }
        return false;
    }

    pub fn get_keys_just_pressed(&self) -> Vec<KeyCode> {
        let mut r = Vec::new();
        
        for key in self.keys_states.iter().filter(|x| (x.1).just_pressed) {
            r.push(key.0.clone());
        }

        return r;
    }

    pub fn get_keys_just_released(&self) -> Vec<KeyCode> {
        let mut r = Vec::new();
        
        for key in self.keys_states.iter().filter(|x| (x.1).just_released) {
            r.push(key.0.clone());
        }

        return r;
    }
    

}