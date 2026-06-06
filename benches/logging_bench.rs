//! Benchmarks for prologger.
//!
//! Run with: `cargo bench --all-features`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use log::Level;
use prologger::formatter::{compact::CompactFormatter, pretty::PrettyFormatter, Format};

#[cfg(feature = "json")]
use prologger::formatter::json::JsonFormatter;

// ─── Formatter Benchmarks ─────────────────────────────────────────────────

fn bench_pretty_formatter(c: &mut Criterion) {
    let formatter = PrettyFormatter::new(false);

    c.bench_function("format_pretty", |b| {
        b.iter(|| {
            let record = log::Record::builder()
                .args(format_args!("Request processed in 42ms"))
                .level(Level::Info)
                .target("bench_app::module")
                .build();
            let output = formatter.format(black_box(&record));
            black_box(output);
        });
    });
}

fn bench_pretty_formatter_colored(c: &mut Criterion) {
    let formatter = PrettyFormatter::new(true);

    c.bench_function("format_pretty_colored", |b| {
        b.iter(|| {
            let record = log::Record::builder()
                .args(format_args!("Request processed in 42ms"))
                .level(Level::Info)
                .target("bench_app::module")
                .build();
            let output = formatter.format(black_box(&record));
            black_box(output);
        });
    });
}

fn bench_compact_formatter(c: &mut Criterion) {
    let formatter = CompactFormatter;

    c.bench_function("format_compact", |b| {
        b.iter(|| {
            let record = log::Record::builder()
                .args(format_args!("Request processed in 42ms"))
                .level(Level::Info)
                .target("bench_app::module")
                .build();
            let output = formatter.format(black_box(&record));
            black_box(output);
        });
    });
}

#[cfg(feature = "json")]
fn bench_json_formatter(c: &mut Criterion) {
    let formatter = JsonFormatter;

    c.bench_function("format_json", |b| {
        b.iter(|| {
            let record = log::Record::builder()
                .args(format_args!("Request processed in 42ms"))
                .level(Level::Info)
                .target("bench_app::module")
                .build();
            let output = formatter.format(black_box(&record));
            black_box(output);
        });
    });
}

// ─── Filter Benchmarks ───────────────────────────────────────────────────

fn bench_filter_global_pass(c: &mut Criterion) {
    let filter = prologger::filter::Filter::new(log::LevelFilter::Debug);
    let metadata = log::Metadata::builder()
        .level(Level::Info)
        .target("bench_app")
        .build();

    c.bench_function("filter_global_pass", |b| {
        b.iter(|| {
            black_box(filter.is_enabled(black_box(&metadata)));
        });
    });
}

fn bench_filter_global_reject(c: &mut Criterion) {
    let filter = prologger::filter::Filter::new(log::LevelFilter::Warn);
    let metadata = log::Metadata::builder()
        .level(Level::Debug)
        .target("bench_app")
        .build();

    c.bench_function("filter_global_reject", |b| {
        b.iter(|| {
            black_box(filter.is_enabled(black_box(&metadata)));
        });
    });
}

fn bench_filter_module_match(c: &mut Criterion) {
    let filter = prologger::filter::Filter::new(log::LevelFilter::Info)
        .with_module("bench_app", log::LevelFilter::Debug)
        .with_module("hyper", log::LevelFilter::Warn)
        .with_module("tokio", log::LevelFilter::Error)
        .with_module("bench_app::db", log::LevelFilter::Trace);

    let metadata = log::Metadata::builder()
        .level(Level::Debug)
        .target("bench_app::api::handlers")
        .build();

    c.bench_function("filter_module_match", |b| {
        b.iter(|| {
            black_box(filter.is_enabled(black_box(&metadata)));
        });
    });
}

// ─── Env Parsing Benchmarks ──────────────────────────────────────────────

fn bench_env_parse_simple(c: &mut Criterion) {
    c.bench_function("env_parse_simple", |b| {
        b.iter(|| {
            let config = prologger::EnvConfig::parse(black_box("debug")).unwrap();
            black_box(config);
        });
    });
}

fn bench_env_parse_complex(c: &mut Criterion) {
    c.bench_function("env_parse_complex", |b| {
        b.iter(|| {
            let config = prologger::EnvConfig::parse(black_box(
                "warn,my_app=debug,hyper=error,tokio=warn,my_app::db=trace",
            ))
            .unwrap();
            black_box(config);
        });
    });
}

// ─── Criterion Groups ────────────────────────────────────────────────────

#[cfg(feature = "json")]
criterion_group!(
    formatters,
    bench_pretty_formatter,
    bench_pretty_formatter_colored,
    bench_compact_formatter,
    bench_json_formatter,
);

#[cfg(not(feature = "json"))]
criterion_group!(
    formatters,
    bench_pretty_formatter,
    bench_pretty_formatter_colored,
    bench_compact_formatter,
);

criterion_group!(
    filters,
    bench_filter_global_pass,
    bench_filter_global_reject,
    bench_filter_module_match,
);

criterion_group!(env_parsing, bench_env_parse_simple, bench_env_parse_complex,);

criterion_main!(formatters, filters, env_parsing);
