use glutin::event::{ElementState, VirtualKeyCode, WindowEvent};
use std::collections::HashMap;

pub struct KeyboardState {
    keydown: HashMap<VirtualKeyCode, bool>,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            keydown: HashMap::new(),
        }
    }

    pub fn is_key_down(&self, key: VirtualKeyCode) -> bool {
        *self.keydown.get(&key).unwrap_or(&false)
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        let WindowEvent::KeyboardInput { input, .. } = event else { return };
        let Some(key) = input.virtual_keycode else { return };

        self.keydown.insert(
            key,
            match input.state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            },
        );
    }
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self::new()
    }
}
