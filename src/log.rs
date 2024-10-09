use crate::Result;

use tracing_subscriber::filter;

#[allow(unused)]
pub struct LogGuard {
    subscriber_guard: tracing::subscriber::DefaultGuard,
}

pub struct LogSubscriber {
    pub(crate) log_subscriber: tracing_subscriber::FmtSubscriber<
        tracing_subscriber::fmt::format::DefaultFields,
        tracing_subscriber::fmt::format::Format,
        filter::EnvFilter,
    >,
}

#[derive(Clone)]
pub struct LogDispatcher {
    pub(crate) dispatcher: tracing::Dispatch,
}

impl LogSubscriber {
    pub fn get_dispatcher(self) -> LogDispatcher {
        LogDispatcher {
            dispatcher: tracing::Dispatch::new(self.log_subscriber),
        }
    }
}

pub fn set_thread_default_subscriber(log_subscriber: LogSubscriber) -> LogGuard {
    let subscriber_guard = tracing::subscriber::set_default(log_subscriber.log_subscriber);
    LogGuard { subscriber_guard }
}

pub fn set_thread_default_dispatcher(log_dispatcher: &LogDispatcher) -> LogGuard {
    let subscriber_guard = tracing::dispatcher::set_default(&log_dispatcher.dispatcher);
    LogGuard { subscriber_guard }
}

pub fn get_subscriber() -> Result<LogSubscriber> {
    let log_subscriber = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        // .with_max_level(Level::TRACE)
        .with_env_filter(filter::EnvFilter::from_default_env())
        // build but do not install the subscriber.
        .finish();
    Ok(LogSubscriber { log_subscriber })
}
