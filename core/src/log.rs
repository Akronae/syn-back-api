use tracing::subscriber::SetGlobalDefaultError;

use crate::config::EnvVar;

fn get_rust_log() -> String {
    EnvVar::RustLog.get().unwrap_or("".into())
}

pub fn get_subscriber() -> impl tracing::Subscriber {
    tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_target(false)
        .with_env_filter(get_rust_log())
        .finish()
}

pub fn init() -> Result<(), SetGlobalDefaultError> {
    tracing::subscriber::set_global_default(get_subscriber())?;
    tracing::info!(rust_log = get_rust_log(), "initialized logging");

    Ok(())
}
