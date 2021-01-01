use crate::events;
use glfw::Context;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    input_handler: events::InputHandler,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW");

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.set_key_polling(true);
        window.make_current();

        Self {
            glfw,
            window,
            events,
            input_handler: events::InputHandler::new(),
        }
    }

    pub fn get_input_handler(&self) -> &events::InputHandler {
        &self.input_handler
    }

    pub fn set_close_requested(&mut self, close_requested: bool) {
        self.window.set_should_close(close_requested);
    }

    pub fn is_close_requested(&self) -> bool {
        self.window.should_close()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn poll_events(&mut self) {
        self.input_handler.clear();
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            log::debug!("Received window event: {:?}", event);
            match event {
                glfw::WindowEvent::Key(key, _, glfw::Action::Press, _) => {
                    self.input_handler.on_key_pressed_event(key)
                }
                glfw::WindowEvent::Key(key, _, glfw::Action::Release, _) => {
                    self.input_handler.on_key_released_event(key)
                }
                _ => log::debug!("Unhandled window event: {:?}", event),
            }
        }
    }
}
