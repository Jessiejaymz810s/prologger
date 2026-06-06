//! Tracing subscriber layer for routing `tracing` events to `prologger`.
//!
//! Enables using `prologger` sinks and formatters natively within a `tracing`
//! subscriber registry.

use crate::logger::ProLogger;
use log::Log;
use tracing_core::{Event, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

/// A `tracing` layer that routes events to a configured `ProLogger`.
pub struct ProloggerLayer {
    logger: ProLogger,
}

impl ProloggerLayer {
    /// Creates a new layer from a configured `ProLogger`.
    pub fn new(logger: ProLogger) -> Self {
        Self { logger }
    }
}

struct FieldVisitor {
    message: String,
    fields: String,
}

impl FieldVisitor {
    fn new() -> Self {
        Self {
            message: String::new(),
            fields: String::new(),
        }
    }
}

impl tracing_core::field::Visit for FieldVisitor {
    fn record_debug(&mut self, field: &tracing_core::Field, value: &dyn std::fmt::Debug) {
        use std::fmt::Write;
        if field.name() == "message" {
            let _ = write!(&mut self.message, "{:?}", value);
        } else {
            let pre = if self.fields.is_empty() { "" } else { " " };
            let _ = write!(&mut self.fields, "{}{}={:?}", pre, field.name(), value);
        }
    }

    fn record_str(&mut self, field: &tracing_core::Field, value: &str) {
        use std::fmt::Write;
        if field.name() == "message" {
            self.message.push_str(value);
        } else {
            let pre = if self.fields.is_empty() { "" } else { " " };
            let _ = write!(&mut self.fields, "{}{}={}", pre, field.name(), value);
        }
    }
}

impl<S: Subscriber> Layer<S> for ProloggerLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let level = tracing_to_log_level(event.metadata().level());
        
        let metadata = log::Metadata::builder()
            .level(level)
            .target(event.metadata().target())
            .build();

        if !self.logger.enabled(&metadata) {
            return;
        }

        let mut visitor = FieldVisitor::new();
        event.record(&mut visitor);

        let final_msg = if visitor.fields.is_empty() {
            visitor.message
        } else if visitor.message.is_empty() {
            visitor.fields
        } else {
            format!("{} {}", visitor.message, visitor.fields)
        };

        let mut builder = log::Record::builder();
        builder
            .level(level)
            .target(event.metadata().target())
            .module_path(event.metadata().module_path())
            .file(event.metadata().file())
            .line(event.metadata().line());

        let args = format_args!("{}", final_msg);
        let record = builder.args(args).build();

        self.logger.log(&record);
    }
}

fn tracing_to_log_level(level: &tracing_core::Level) -> log::Level {
    match *level {
        tracing_core::Level::ERROR => log::Level::Error,
        tracing_core::Level::WARN => log::Level::Warn,
        tracing_core::Level::INFO => log::Level::Info,
        tracing_core::Level::DEBUG => log::Level::Debug,
        tracing_core::Level::TRACE => log::Level::Trace,
    }
}
