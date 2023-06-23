use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init(filter: String, log_dir: String, log_file: String) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter));
    let file_appender = rolling::never(log_dir, log_file);
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking_appender);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .init()
}