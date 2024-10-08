use tracing_subscriber::filter;

//not - can't use     tracing_subscriber::fmt::init();
//anything that pins itself to memory is incompatible with hot-reload and defeats the purpose

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(all(feature = "log", not(feature = "log-file")))]
pub struct LogGuard {
    subscriber_guard: tracing::subscriber::DefaultGuard,
}

#[cfg(all(feature = "log", feature = "log-file"))]
pub struct LogGuard {
    subscriber_guard: tracing::subscriber::DefaultGuard,
    file_flush_guard: tracing_appender::non_blocking::WorkerGuard,
}

#[cfg(all(feature = "log", not(feature = "log-file")))]
pub struct LogSubscriber {
    log_subscriber: tracing_subscriber::FmtSubscriber<
        tracing_subscriber::fmt::format::DefaultFields,
        tracing_subscriber::fmt::format::Format,
        filter::EnvFilter,
    >,
}

#[cfg(all(feature = "log", not(feature = "log-file")))]
pub fn get_subscription() -> Result<LogGuard> {
    let log_subscriber = get_subscriber()?;
    Ok(get_subscription_for_subscriber(log_subscriber))
}

#[cfg(all(feature = "log", not(feature = "log-file")))]
pub fn get_subscription_for_subscriber(log_subscriber: LogSubscriber) -> LogGuard {
    let subscriber_guard = tracing::subscriber::set_default(log_subscriber.log_subscriber);
    LogGuard { subscriber_guard }
}

#[cfg(all(feature = "log", not(feature = "log-file")))]
pub fn get_subscriber() -> Result<LogSubscriber> {
    let log_subscriber = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        // .with_max_level(Level::TRACE)
        .with_env_filter(filter::EnvFilter::from_default_env())
        // build but do not install the subscriber.
        .finish();
    Ok(LogSubscriber { log_subscriber })
}

#[cfg(all(feature = "log", feature = "log-file"))]
pub struct LogSubscriber {
    file_flush_guard: tracing_appender::non_blocking::WorkerGuard,
    log_subscriber: tracing_subscriber::FmtSubscriber<
        tracing_subscriber::fmt::format::DefaultFields,
        tracing_subscriber::fmt::format::Format,
        tracing_subscriber::EnvFilter,
        tracing_appender::non_blocking::NonBlocking,
    >,
}

#[cfg(all(feature = "log", feature = "log-file"))]
pub fn get_subscription() -> Result<LogGuard> {
    let log_subscriber = get_subscriber()?;
    Ok(get_subscription_for_subscriber(log_subscriber))
}

#[cfg(all(feature = "log", feature = "log-file"))]
pub fn get_subscription_for_subscriber(log_subscriber: LogSubscriber) -> LogGuard {
    let subscriber_guard = tracing::subscriber::set_default(log_subscriber.log_subscriber);
    let file_flush_guard = log_subscriber.file_flush_guard;

    LogGuard {
        subscriber_guard,
        file_flush_guard,
    }
}

#[cfg(all(feature = "log", feature = "log-file"))]
pub fn get_subscriber() -> Result<LogSubscriber> {
    let log_dir = std::env::var("LOG_PATH").map_err(|e| format!("LOG_PATH is not set {:?}", e))?;
    let log_prefix =
        std::env::var("LOG_PREFIX").map_err(|e| format!("LOG_PREFIX is not set {:?}", e))?;
    let file_appender = tracing_appender::rolling::daily(log_dir, log_prefix);
    let (non_blocking, file_flush_guard) = tracing_appender::non_blocking(file_appender);
    let log_subscriber = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        // .with_max_level(Level::TRACE)
        .with_env_filter(filter::EnvFilter::from_default_env())
        .with_writer(non_blocking)
        // build but do not install the subscriber.
        .finish();
    Ok(LogSubscriber {
        file_flush_guard,
        log_subscriber,
    })
}
