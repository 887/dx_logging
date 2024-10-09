//not - can't use     tracing_subscriber::fmt::init();
//anything that pins itself to memory is incompatible with hot-reload and defeats the purpose

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(not(feature = "log-file"))]
mod log;
#[cfg(not(feature = "log-file"))]
pub use log::*;

#[cfg(feature = "log-file")]
mod log_file;
#[cfg(feature = "log-file")]
pub use log_file::*;

pub fn subscribe_thread_with_default() -> Result<LogGuard> {
    let log_subscriber = get_subscriber()?;
    Ok(set_thread_default_subscriber(log_subscriber))
}

// pub fn set_global_default_subscriber(log_subscriber: LogSubscriber) -> Result<()> {
//     tracing::subscriber::set_global_default(log_subscriber.log_subscriber)
//         .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
// }

pub fn set_global_default_dispatcher(log_dispatcher: LogDispatcher) -> Result<()> {
    tracing::dispatcher::set_global_default(log_dispatcher.dispatcher)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
}
