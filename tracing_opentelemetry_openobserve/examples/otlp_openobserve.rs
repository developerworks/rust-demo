use std::collections::HashMap;
use opentelemetry::trace::Tracer;
use opentelemetry_api::{Key, KeyValue};
use opentelemetry_api::global::shutdown_tracer_provider;
use opentelemetry_api::trace::TraceContextExt;
use opentelemetry_otlp::{WithExportConfig};
use tracing::{debug, error, info, span, trace, warn, Level};
use opentelemetry_sdk::{trace as sdktrace, Resource, runtime};
use uuid::Uuid;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {

    // let subscriber = tracing_subscriber::fmt()
    //     .with_ansi(true)
    //     .with_level(true)
    //     .with_file(true)
    //     .with_line_number(true).with_file(true)
    //     ;


    // First, create a OTLP exporter builder. Configure it as you need.
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_env()
        // .with_export_config(
        //     ExportConfig::default()
        // )
        .with_headers(authentication_headers())
        .with_endpoint("http://localhost:5080/api/developerworks/traces")
        ;
    // Then pass it into pipeline builder
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(
            sdktrace::config()
                .with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "basic-otlp-tracing-example",
            )])),
        )
        .install_batch(runtime::Tokio)?;

    tracer.in_span("doing_work", |cx| {
        // Traced app logic here...
        let span = cx.span();
        span.add_event(
            "Nice operation!".to_string(),
            vec![Key::new("bogons").i64(100)],
        );
        span.set_attributes(vec![
            KeyValue::new(Key::from_static_str("request_id"), Uuid::new_v4().to_string()),
            KeyValue::new(Key::from_static_str("user_id"), "1"),
            KeyValue::new(Key::from_static_str("user_name"), "developerworks")
        ]);
        info!(target: "my-target", "hello from {}. My price is {}. I am also inside a Span!", "banana", 2.99);

        app();
    });

    shutdown_tracer_provider();

    Ok(())
}

fn authentication_headers() -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert(
        "User-Agent".to_string(),
        format!("OTel OTLP Exporter Rust/{}", env!("CARGO_PKG_VERSION")),
    );
    headers.insert(
        String::from("Authorization"),
        String::from("Basic cm9vdEBleGFtcGxlLmNvbTpJYXZXS1JoUkRXWEhkRFN5"),
    );
    headers
}

fn app() {
    let span = span!(Level::TRACE, "app logs");
    let _enter = span.enter();
    debug!("debug hello tracing subscriber");
    info!("info hello tracing subscriber");
    warn!("warn hello tracing subscriber");
    error!("error hello tracing subscriber");
    trace!("trace hello tracing subscriber");
}