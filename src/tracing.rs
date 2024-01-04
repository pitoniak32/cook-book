use std::env;

use crate::error::Result;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

pub fn init_tracing() -> Result<()> {
    Registry::default()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cbk=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .with(
            tracing_opentelemetry::layer().with_tracer(
                opentelemetry_otlp::new_pipeline()
                    .tracing()
                    .with_exporter(
                        opentelemetry_otlp::new_exporter()
                            .tonic()
                            .with_endpoint(String::from("https://localhost:4317")),
                    )
                    .with_trace_config(opentelemetry_sdk::trace::config().with_resource(
                        Resource::new(vec![KeyValue::new(
                            opentelemetry_semantic_conventions::resource::SERVICE_NAME.to_string(),
                            env::var("CARGO_PKG_NAME").expect("crate name should be set by cargo"),
                        )]),
                    ))
                    .install_batch(opentelemetry_sdk::runtime::Tokio)
                    .expect("Failed creating the tracer!"),
            ),
        )
        .init();

    Ok(())
}
