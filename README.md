# 🪵 Prologger

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.56%2B-orange.svg)](https://www.rust-lang.org)

A production-grade, ergonomic Rust logging library with colored output, file rotation, and structured formatting.

Prologger implements the [`log`](https://docs.rs/log) crate facade, so you can use the standard `log::info!()`, `log::warn!()`, etc. macros throughout your codebase.

## ✨ Features

- **`log` crate compatible** — Drop-in replacement, works with the entire Rust ecosystem
- **Colored console output** — Level-based ANSI coloring with auto-detection
- **Multiple formatters** — Pretty (human), JSON (machine), Compact (minimal)
- **File logging** — Write to files with size-based rotation
- **Multi-sink** — Route logs to multiple destinations simultaneously
- **Builder API** — Fluent configuration with sensible defaults
- **Module filtering** — Fine-grained per-module log level control
- **Thread-safe** — All components are `Send + Sync`
- **Lightweight** — Only `log` as a required dependency

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
prologger = { git = "https://github.com/Jessiejaymz810s/prologger.git" }
log = "0.4"
```

Then initialize and start logging:

```rust
use log::{info, warn, error, debug};

fn main() {
    prologger::init();

    info!("Application started");
    debug!("This won't show at default Info level");
    warn!("Low disk space");
    error!("Connection failed");
}
```

## 🔧 Builder API

For fine-grained control:

```rust
use log::LevelFilter;
use prologger::ProLoggerBuilder;

ProLoggerBuilder::new()
    .with_level(LevelFilter::Debug)
    .with_console_default()
    .with_module_filter("hyper", LevelFilter::Warn)
    .with_module_filter("my_app::db", LevelFilter::Debug)
    .init()
    .unwrap();
```

## 📁 File Logging with Rotation

```rust
use prologger::{ProLoggerBuilder, RotationConfig};

ProLoggerBuilder::new()
    .with_console_default()
    .with_rotating_file(
        "logs/app.log",
        RotationConfig::megabytes(10, 5), // 10MB per file, keep 5 backups
    )
    .init()
    .unwrap();
```

## 📊 JSON Output

Perfect for log aggregation tools (Elasticsearch, Loki, CloudWatch):

```rust
use prologger::{ProLoggerBuilder, FormatterType};

ProLoggerBuilder::new()
    .with_formatter(FormatterType::Json)
    .with_console_default()
    .init()
    .unwrap();
```

Output:
```json
{"timestamp":"2026-06-05T20:15:00.123Z","level":"INFO","target":"my_app","message":"Service started on port 8080"}
```

## 📦 Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `color` | ✅ | ANSI colored terminal output |
| `file`  | ✅ | File logging with size-based rotation |
| `json`  | ❌ | JSON structured output formatter |
| `full`  | ❌ | Enables all features |

Enable specific features:

```toml
[dependencies]
prologger = { git = "https://github.com/Jessiejaymz810s/prologger.git", features = ["full"] }
```

## 🎨 Output Formats

### Pretty (default)
```
2026-06-05 20:15:00.123 INFO  [my_app::api] Request processed successfully
2026-06-05 20:15:00.456 WARN  [my_app::db]  Connection pool running low
2026-06-05 20:15:00.789 ERROR [my_app::api] Failed to process request
```

### Compact
```
I: Request processed successfully
W: Connection pool running low
E: Failed to process request
```

### JSON
```json
{"timestamp":"2026-06-05T20:15:00.123Z","level":"INFO","target":"my_app::api","message":"Request processed successfully"}
```

## 🏗️ Architecture

```
prologger
├── src/
│   ├── lib.rs          # Public API & convenience functions
│   ├── builder.rs      # Builder pattern configuration
│   ├── logger.rs       # log::Log trait implementation
│   ├── filter.rs       # Level filtering engine
│   ├── color.rs        # ANSI color support
│   ├── rotation.rs     # File rotation logic
│   ├── formatter/      # Output formatters
│   │   ├── pretty.rs   # Human-readable format
│   │   ├── json.rs     # JSON format
│   │   └── compact.rs  # Minimal format
│   └── sink/           # Output destinations
│       ├── console.rs  # Terminal output
│       └── file.rs     # File output
├── examples/
└── tests/
```

## 📄 License

MIT — see [LICENSE](LICENSE) for details.
