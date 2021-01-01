mod events;
mod window;

fn main() {
    setup_logging();

    let mut window = window::Window::new(1280, 720, "Rust Vulkan Example");
    log::info!("Window created");

    while !window.is_close_requested() {
        window.swap_buffers();
        window.poll_events();
        if window
            .get_input_handler()
            .was_key_pressed(glfw::Key::Escape)
        {
            window.set_close_requested(true);
        }

        if window.get_input_handler().is_key_down(glfw::Key::Space) {
            log::info!("Jumping!");
        }

        if window.get_input_handler().was_key_released(glfw::Key::S) {
            log::info!("Stopped moving forward");
        }
    }

    log::info!("Shutting down...");
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
                .open("logs/app.log")
                .expect("Failed to create app log file"),
        )
        .apply()
        .unwrap();

    log::debug!("Logging initialized");
}
