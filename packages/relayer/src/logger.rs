use lazy_static::lazy_static;
use slog::{o, Drain, Level};
use slog_async;
use slog_json;
use slog_term;
use std::fs::OpenOptions;

lazy_static! {
    pub static ref LOG: slog::Logger = init_logger();
}

fn init_logger() -> slog::Logger {
    let log_terminal_decorator = slog_term::TermDecorator::new().build();
    let log_terminal_drain = slog_term::FullFormat::new(log_terminal_decorator)
        .build()
        .fuse();

    let log_path = "app.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();
    let log_file_drain = slog_json::Json::default(file).fuse();

    let log_drain = slog_async::Async::new(
        (slog::Duplicate(log_terminal_drain, log_file_drain))
            .filter_level(Level::Trace)
            .fuse(),
    )
    .build()
    .fuse();

    slog::Logger::root(log_drain, o!("version" => env!("CARGO_PKG_VERSION")))
}
