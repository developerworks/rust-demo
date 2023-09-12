use opentelemetry_api::{KeyValue, trace::Tracer};
use opentelemetry_sdk::{trace::{self, RandomIdGenerator, Sampler}, Resource};
use opentelemetry_sdk::metrics as sdkmetrics;

use opentelemetry_otlp::{Protocol, WithExportConfig, ExportConfig};
use std::time::Duration;
use tonic::metadata::{MetadataMap, MetadataValue};
use opentelemetry::metrics;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use tracing_subscriber::util::SubscriberInitExt;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut map = MetadataMap::with_capacity(3);

    map.insert("x-host", "example.com".parse().unwrap());
    map.insert("x-number", "123".parse().unwrap());
    map.insert_bin("trace-proto-bin", MetadataValue::from_bytes(b"[binary data]"));

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())

        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317")
                .with_timeout(Duration::from_secs(3))
                .with_metadata(map)
        )
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOff)
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new("service.name", "example")])),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    let export_config = ExportConfig {
        endpoint: "http://localhost:4317".to_string(),
        timeout: Duration::from_secs(3),
        protocol: Protocol::Grpc,
    };

    let meter = opentelemetry_otlp::new_pipeline()
        .metrics(tokio::spawn)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
            // can also config it using with_* functions like the tracing part above.
        )
        // .with_stateful(true)
        .with_period(Duration::from_secs(3))
        .with_timeout(Duration::from_secs(10))
        .with_aggregation_selector()
        .build();

    tracer.in_span("doing_work", |cx| {
        // Traced app logic here...
    });


    Ok(())
}

fn init_metrics() -> metrics::Result<sdkmetrics::MeterProvider> {
    let export_config = ExportConfig {
        endpoint: "http://localhost:4318/v1/metrics".to_string(),
        ..ExportConfig::default()
    };
    opentelemetry_otlp::new_pipeline()
        .metrics(opentelemetry_sdk::runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_export_config(export_config),
        )
        .build()
}


fn init_jaeger_tracing_pipline(registry: &Registry) {
    // 导出器
    // with_env 从支持的环境变量中配置它
    // OTEL_EXPORTER_OTLP_ENDPOINT=https://api.honeycomb.io:443
    // OTEL_EXPORTER_OTLP_HEADERS="x-honeycomb-team=your-api-key"
    let jaeger_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://192.168.0.23:4317")
        .with_env();


    // 跟踪器
    let jaeger_tracer = opentelemetry_otlp::new_pipeline()
        // 跟踪管道
        .tracing()
        // 到初器
        .with_exporter(jaeger_exporter)
        // 批量
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Couldn't create OTLP tracer");
    let jaeger_layer = tracing_opentelemetry::layer().with_tracer(jaeger_tracer);
    //
    // let openobserve_exporter = opentelemetry_otlp::new_exporter()
    //     .tonic()
    //     .with_endpoint("http://192.168.0.23:5081")
    //     .with_env();
    // let openobserve_tracer = opentelemetry_otlp::new_pipeline()
    //     .tracing()
    //     .with_exporter(openobserve_exporter)
    //     .install_batch(opentelemetry::runtime::Tokio)
    //     .expect("Could not create OpenObserve tracer");
    // let openobserve_layer = tracing_opentelemetry::layer().with_tracer(openobserve_tracer);

    // 注册表
    registry
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(jaeger_layer)
        // .with(openobserve_layer)
        .init();
}