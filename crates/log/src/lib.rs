use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry::runtime::Tokio;

pub fn init(filter: String, log_dir: String, log_file: String, jaeger_url: Option<String>, s_name: Option<String>) -> bool {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter));
    let file_appender = rolling::never(log_dir, log_file);
    let (non_blocking_appender, _guard) = non_blocking(file_appender);

    if let Some(url) = jaeger_url {
        let mut agnet = opentelemetry_jaeger::new_agent_pipeline().with_endpoint(url);
        if let Some(name) = s_name {
            agnet = agnet.with_service_name(name);
        }
        let tracer = match agnet.install_batch(Tokio) {
            Err(_) => {
                return false
            },
            Ok(t) => t
        };
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking_appender);
        tracing_subscriber::registry().with(env_filter).with(telemetry).with(file_layer).init();
    }
    else{
        let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking_appender);
        tracing_subscriber::registry().with(env_filter).with(file_layer).init();
    }

    return true
}