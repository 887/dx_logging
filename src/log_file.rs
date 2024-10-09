use std::sync::Arc;

use crate::Result;

use tracing_subscriber::filter;

#[allow(unused)]
pub struct LogGuard {
    subscriber_guard: tracing::subscriber::DefaultGuard,
    file_flush_guard: Arc<tracing_appender::non_blocking::WorkerGuard>,
}

pub struct LogSubscriber {
    file_flush_guard: Arc<tracing_appender::non_blocking::WorkerGuard>,
    pub(crate) log_subscriber: tracing_subscriber::FmtSubscriber<
        tracing_subscriber::fmt::format::DefaultFields,
        tracing_subscriber::fmt::format::Format,
        tracing_subscriber::EnvFilter,
        tracing_appender::non_blocking::NonBlocking,
    >,
}

#[derive(Clone)]
pub struct LogDispatcher {
    pub(crate) dispatcher: tracing::Dispatch,
    file_flush_guard: Arc<tracing_appender::non_blocking::WorkerGuard>,
}

impl LogSubscriber {
    pub fn get_dispatcher(self) -> LogDispatcher {
        let file_flush_guard = self.file_flush_guard;
        LogDispatcher {
            dispatcher: tracing::Dispatch::new(self.log_subscriber),
            file_flush_guard,
        }
    }
}

pub fn set_thread_default_subscriber(log_subscriber: LogSubscriber) -> LogGuard {
    let subscriber_guard = tracing::subscriber::set_default(log_subscriber.log_subscriber);
    let file_flush_guard = log_subscriber.file_flush_guard;

    LogGuard {
        subscriber_guard,
        file_flush_guard,
    }
}

pub fn set_thread_default_dispatcher(log_dispatcher: &LogDispatcher) -> LogGuard {
    let subscriber_guard = tracing::dispatcher::set_default(&log_dispatcher.dispatcher);
    let file_flush_guard = log_dispatcher.file_flush_guard.clone();

    LogGuard {
        subscriber_guard,
        file_flush_guard,
    }
}

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

    let file_flush_guard = Arc::new(file_flush_guard);

    Ok(LogSubscriber {
        file_flush_guard,
        log_subscriber,
    })
}
