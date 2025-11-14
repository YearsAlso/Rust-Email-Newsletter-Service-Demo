use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber, Registry};
use tracing_subscriber::layer::SubscriberExt;

pub fn get_subscriber (
    name: String,
    env_filter: String
) -> impl tracing::Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formating_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formating_layer);
    subscriber
}

pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to init LogTracer");
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");
}