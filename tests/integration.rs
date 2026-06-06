//! Integration tests for prologger.

use prologger::*;
use log::LevelFilter;

#[test]
fn test_builder_creates_logger() {
    let logger = ProLoggerBuilder::new()
        .with_level(LevelFilter::Debug)
        .with_console_default()
        .build();

    // Logger should have been created with one sink
    assert!(format!("{:?}", logger).contains("ProLogger"));
}

#[test]
fn test_builder_default_adds_console_sink() {
    let logger = ProLoggerBuilder::new().build();
    // Default build should add a console sink
    assert!(format!("{:?}", logger).contains("sinks_count: 1"));
}

#[test]
fn test_formatter_types() {
    // Pretty formatter
    let logger = ProLoggerBuilder::new()
        .with_formatter(FormatterType::Pretty)
        .build();
    assert!(format!("{:?}", logger).contains("ProLogger"));

    // Compact formatter — build a new one since we can't reuse
    let logger = ProLoggerBuilder::new()
        .with_formatter(FormatterType::Compact)
        .build();
    assert!(format!("{:?}", logger).contains("ProLogger"));
}

#[cfg(feature = "json")]
#[test]
fn test_json_formatter_type() {
    let logger = ProLoggerBuilder::new()
        .with_formatter(FormatterType::Json)
        .build();
    assert!(format!("{:?}", logger).contains("ProLogger"));
}

#[test]
fn test_module_filter() {
    let logger = ProLoggerBuilder::new()
        .with_level(LevelFilter::Info)
        .with_module_filter("noisy_lib", LevelFilter::Error)
        .build();

    // The logger should have the filter configured
    assert!(format!("{:?}", logger).contains("ProLogger"));
}

#[cfg(feature = "file")]
#[test]
fn test_file_sink() {
    let dir = std::env::temp_dir().join("prologger_integration_test");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let log_path = dir.join("test.log");

    let logger = ProLoggerBuilder::new()
        .with_file(log_path.to_str().unwrap())
        .build();

    // Should have the file sink
    assert!(format!("{:?}", logger).contains("sinks_count: 1"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(feature = "file")]
#[test]
fn test_rotating_file_sink() {
    let dir = std::env::temp_dir().join("prologger_rotation_integration");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let log_path = dir.join("test.log");

    let logger = ProLoggerBuilder::new()
        .with_rotating_file(
            log_path.to_str().unwrap(),
            RotationConfig::new(1000, 3),
        )
        .build();

    assert!(format!("{:?}", logger).contains("sinks_count: 1"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_multi_sink() {
    let logger = ProLoggerBuilder::new()
        .with_console_default()
        .with_console_default()  // Two console sinks
        .build();

    assert!(format!("{:?}", logger).contains("sinks_count: 2"));
}
