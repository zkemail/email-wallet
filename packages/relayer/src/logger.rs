use file_rotate::{
    compression::Compression,
    suffix::{AppendTimestamp, FileLimit},
    ContentLimit, FileRotate,
};
use lazy_static::lazy_static;
use slog::{o, Drain, Level};
use slog_async;
use slog_json;
use slog_term;

lazy_static! {
    pub static ref LOG: slog::Logger = init_logger();
}

fn init_logger() -> slog::Logger {
    let log_terminal_decorator = slog_term::TermDecorator::new().build();
    let log_terminal_drain = slog_term::FullFormat::new(log_terminal_decorator)
        .build()
        .fuse();

    let directory = std::path::Path::new("logs");
    let log_path = directory.join("relayer.log");
    let file_rotate = FileRotate::new(
        log_path.clone(),
        AppendTimestamp::default(FileLimit::MaxFiles(1_000_000)),
        ContentLimit::Bytes(5_000_000),
        Compression::None,
        #[cfg(unix)]
        None,
    );
    let log_file_drain = slog_json::Json::default(file_rotate).fuse();

    let log_drain = slog_async::Async::new(
        (slog::Duplicate(log_terminal_drain, log_file_drain))
            .filter_level(Level::Trace)
            .fuse(),
    )
    .build()
    .fuse();

    slog::Logger::root(log_drain, o!("version" => env!("CARGO_PKG_VERSION")))
}
