//! Logger that forwards to env_logger and captures warnings for display in
//! the UI.
//!
//! The engine reports recoverable misconfigurations (unparseable percent
//! values, missing income links, clamped years, skipped employer matches)
//! via `log::warn!`.  With plain env_logger those are invisible unless the
//! user sets RUST_LOG — which defeats the purpose of the warnings.  This
//! logger tees warn-and-above records into a buffer the app shows in the
//! sidebar after each analysis run.

use std::sync::{Arc, Mutex};

/// Shared buffer of warning messages captured since the last `take`
#[derive(Clone, Default)]
pub struct WarningBuffer(Arc<Mutex<Vec<String>>>);

impl WarningBuffer {
    /// Remove and return all captured warnings
    pub fn take(&self) -> Vec<String> {
        match self.0.lock() {
            Ok(mut buf) => std::mem::take(&mut *buf),
            Err(_) => Vec::new(),
        }
    }
    /// Discard any captured warnings
    pub fn clear(&self) {
        if let Ok(mut buf) = self.0.lock() {
            buf.clear();
        }
    }
    fn push(&self, message: String) {
        if let Ok(mut buf) = self.0.lock() {
            // The simulation logs the same warning once per simulated year;
            // collapse duplicates so the UI shows each problem once
            if !buf.contains(&message) {
                buf.push(message);
            }
        }
    }
}

struct UiLogger {
    inner: env_logger::Logger,
    warnings: WarningBuffer,
}

impl log::Log for UiLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Warn || self.inner.enabled(metadata)
    }
    fn log(&self, record: &log::Record) {
        if record.level() <= log::Level::Warn {
            self.warnings.push(format!("{}", record.args()));
        }
        if self.inner.enabled(record.metadata()) {
            self.inner.log(record);
        }
    }
    fn flush(&self) {
        self.inner.flush();
    }
}

/// Install the UI logger (respecting RUST_LOG for console output) and return
/// the shared warning buffer
pub fn init() -> WarningBuffer {
    let inner = env_logger::Builder::from_default_env().build();
    let warnings = WarningBuffer::default();
    let max_level = inner.filter().max(log::LevelFilter::Warn);
    log::set_boxed_logger(Box::new(UiLogger {
        inner,
        warnings: warnings.clone(),
    }))
    .expect("logger already installed");
    log::set_max_level(max_level);
    warnings
}
