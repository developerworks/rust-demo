use once_cell::sync::Lazy;

use opentelemetry::trace::{TraceContextExt, TraceError, Tracer};
use opentelemetry::{global, Key, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace as sdktrace;
use std::collections::HashMap;

const LEMONS_KEY: Key = Key::from_static_str("ex.com/lemons");
const ANOTHER_KEY: Key = Key::from_static_str("ex.com/another");

static COMMON_ATTRIBUTES: Lazy<[KeyValue; 4]> = Lazy::new(|| {
    [
        LEMONS_KEY.i64(10),
        KeyValue::new("A", "1"),
        KeyValue::new("B", "2"),
        KeyValue::new("C", "3"),
    ]
});

#[tokio::main]
async fn main() {
    let _ = init_tracer();
    let tracer = global::tracer("ex.com/basic");
    tracer.in_span("operation", |cx| {
        let span = cx.span();
        span.add_event(
            "Nice operation!".to_string(),
            vec![Key::new("bogons").i64(100)],
        );
        span.set_attribute(ANOTHER_KEY.string("yes"));

        tracer.in_span("Sub operation...", |cx| {
            let span = cx.span();
            span.set_attribute(LEMONS_KEY.string("five"));

            span.add_event("Sub span event", vec![]);
        });
    });

    sum(1, 2);
}

// A function of sum two integers
#[tracing::instrument]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(unused)]
fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_headers(authentication_headers())
                .with_endpoint("http://localhost:5080/api/default/traces"),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
}

fn authentication_headers() -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert(
        "User-Agent".to_string(),
        format!("OTel OTLP Exporter Rust/{}", env!("CARGO_PKG_VERSION")),
    );
    headers.insert(
        String::from("Authorization"),
        String::from("Basic cm9vdEBleGFtcGxlLmNvbTpyb290"),
    );
    headers
}
