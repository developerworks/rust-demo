use std::fs::OpenOptions;
use chrono::{DateTime, FixedOffset, Local};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::new_exporter;
use opentelemetry_sdk::trace::TracerProvider;
use tracing::{debug, error, info, span, trace, warn, Level};
use tracing_subscriber::fmt::format::{FmtSpan, Writer};
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::fmt;
use tracing_subscriber::util::SubscriberInitExt;

struct FixedOffsetTimeFormatter;

impl FormatTime for FixedOffsetTimeFormatter {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(
            w,
            "{}",
            DateTime::<FixedOffset>::from_naive_utc_and_offset(
                chrono::Utc::now().naive_utc(),
                FixedOffset::east_opt(0).unwrap(),
            )
                .format("%Y-%m-%d %H:%M:%S")
        )
    }
}

struct LocalFormat;

impl FormatTime for LocalFormat {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(
            w,
            "{}",
            DateTime::<Local>::from(Local::now()).format("%Y-%m-%d %H:%M:%S")
        )
    }
}

fn main() {
    init_tracer();
    logs();
}

fn logs() {
    let span = span!(Level::TRACE, "span logs");
    let _enter = span.enter();
    debug!("debug hello tracing subscriber");
    info!("info hello tracing subscriber");
    warn!("warn hello tracing subscriber");
    error!("error hello tracing subscriber");
    trace!("trace hello tracing subscriber");
}

fn init_tracer() {
    // 控制台输出
    let console_layer = fmt::layer()
        .with_timer(LocalFormat)
        .with_level(true)
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_span_events(FmtSpan::FULL);

    // 文件输出
    let file_layer = fmt::layer()
        // .with_filter()
        .with_timer(FixedOffsetTimeFormatter)
        // .event_format(Format::default().with_level(true))
        .with_level(true)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_span_events(FmtSpan::FULL)
        .json()
        .with_writer(
            // std::fs::File::create("./logs/log.json").unwrap(),
            OpenOptions::new().append(true).open("./logs/log.json").unwrap()
        );

    // 遥测后端
    let telemetry_layer = fmt::layer()
        .with_timer(LocalFormat)
        .with_level(true)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_span_events(FmtSpan::FULL);


    // 直接初始化
    let _registry = tracing_subscriber::registry()
        // 使用 RUST_LOG=info 过滤, 例如:
        // RUST_LOG=info cargo run --example fmt
        .with(tracing_subscriber::EnvFilter::from_default_env())
        // 控制台
        .with(console_layer)
        // 文件
        .with(file_layer)
        // 遥测
        .with(telemetry_layer)
        .init();

    // 如果使用全局, 则需要取消上面一行最后的 .init()
    // tracing::subscriber::set_global_default(registry).expect("Failed to set default subscriber");
}

fn init_oltp_telemetry() {
    let exporter = new_exporter();
    let provider = TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();
    let tracer = provider.tracer("readme_example");
    tracer.in_span("doing_work", |cx| {
        // Traced app logic here...
    });
}
