use glfw::{Action, Context, Key};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

fn main() {
    setup_logging();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW");

    let (mut window, events) = glfw
        .create_window(1280, 720, "Rust Vulkan Example", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.make_current();

    log::info!("Window created");

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    log::info!("Shutting down...");
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            log::info!("'Escape' pressed");
            window.set_should_close(true);
        }
        _ => {}
    }
}

fn setup_logging() {
    // Use Debug level logging if debugging, otherwise Info level
    let level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    let file_path = "logs/main.log";
    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
    let pattern = "{d(%Y-%m-%d %H:%M:%S.%3f)} [{h({l:>5})}] {M}: {m}{n}";

    // Build a stderr logger
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    // Logging to log file
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(level),
        )
        .unwrap();

    // Use handle to change log levels at runtime
    let _handle = log4rs::init_config(config).expect("Failed to initialize logging");

    log::debug!("Logging initialized");
}
