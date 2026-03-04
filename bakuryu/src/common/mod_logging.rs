use fern::Dispatch;
use log::LevelFilter;
use std::fs::OpenOptions;

pub fn init_logging() {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(
            OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open("server.log")
                .expect("Failed to open server.log"),
        )
        .apply()
        .expect("Failed to initialize logger");
}
