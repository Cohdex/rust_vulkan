use glfw::{Action, Context, Key};

fn main() {
    setup_logging();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW");

    let (mut window, events) = glfw
        .create_window(1280, 720, "Rust Vulkan Example", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();

    log::info!("Window created");

    while !window.should_close() {
        window.swap_buffers();
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
        _ => log::debug!("Unhandled event: {:?}", event),
    }
}

fn setup_logging() {
    // Use Trace level logging if debugging, otherwise Info level
    let level = if cfg!(debug_assertions) {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Info
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{:>5}] {}: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .chain(
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("logs/main.log")
                .expect("Failed to create main log file"),
        )
        .apply()
        .unwrap();

    log::debug!("Logging initialized");
}
