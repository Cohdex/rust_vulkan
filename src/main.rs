use glfw::{Action, Context, Key};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
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
    // Use Debug level logging to stdout if debugging, otherwise Info level
    let stdout_level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    let file_path = "logs/main.log";
    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
    let pattern = "{d(%Y-%m-%d %H:%M:%S.%3f)} [{h({l:>5})}] {M}: {m}{n}";

    // Build a stderr logger.
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(file_path)
        .unwrap();

    // Log Debug level output to file and the programmatically specified level to stdout.
    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Info)))
                .build("logfile", Box::new(logfile)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(stdout_level)))
                .build("stdout", Box::new(stdout)),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(log::LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config).expect("Failed to initialize logging");

    log::debug!("Logging initialized");
}
