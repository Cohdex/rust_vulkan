use std::collections::HashSet;

#[derive(Default)]
pub struct InputHandler {
    down_keys: HashSet<glfw::Key>,
    pressed_keys: HashSet<glfw::Key>,
    released_keys: HashSet<glfw::Key>,
}

impl InputHandler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_key_down(&self, key: glfw::Key) -> bool {
        self.down_keys.get(&key).is_some()
    }

    pub fn was_key_pressed(&self, key: glfw::Key) -> bool {
        self.pressed_keys.get(&key).is_some()
    }

    pub fn was_key_released(&self, key: glfw::Key) -> bool {
        self.released_keys.get(&key).is_some()
    }

    pub fn on_key_pressed_event(&mut self, key: glfw::Key) {
        self.down_keys.insert(key);
        self.pressed_keys.insert(key);
    }

    pub fn on_key_released_event(&mut self, key: glfw::Key) {
        self.down_keys.remove(&key);
        self.released_keys.insert(key);
    }

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
    }
}
