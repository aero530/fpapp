//! Logger that forwards to env_logger and captures warnings for display in
//! the UI.
//!
//! The engine reports recoverable misconfigurations (unparseable percent
//! values, missing income links, clamped years, skipped employer matches)
//! via `log::warn!`.  With plain env_logger those are invisible unless the
//! user sets RUST_LOG — which defeats the purpose of the warnings.  This
//! logger tees warn-and-above records into a buffer the app shows in the
//! sidebar after each analysis run.
//!
//! Console output goes to stderr through env_logger natively, and to the
//! browser console through eframe's `WebLogger` on the web.

use std::sync::{Arc, Mutex};

/// Logger the captured records are forwarded on to.
#[cfg(not(target_arch = "wasm32"))]
type ConsoleLogger = env_logger::Logger;
#[cfg(target_arch = "wasm32")]
type ConsoleLogger = eframe::WebLogger;

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
        // Hard cap as a backstop; deduplication keeps the buffer tiny in practice
        const MAX_WARNINGS: usize = 100;
        if let Ok(mut buf) = self.0.lock() {
            // The simulation logs the same warning once per simulated year;
            // collapse duplicates so the UI shows each problem once
            if buf.len() < MAX_WARNINGS && !buf.contains(&message) {
                buf.push(message);
            }
        }
    }
}

struct UiLogger {
    inner: ConsoleLogger,
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

/// Install the UI logger (respecting RUST_LOG for console output where there is
/// an environment to read it from) and return the shared warning buffer
pub fn init() -> WarningBuffer {
    let (inner, console_level) = console_logger();
    let warnings = WarningBuffer::default();
    let max_level = console_level.max(log::LevelFilter::Warn);
    log::set_boxed_logger(Box::new(UiLogger {
        inner,
        warnings: warnings.clone(),
    }))
    .expect("logger already installed");
    log::set_max_level(max_level);
    warnings
}

#[cfg(not(target_arch = "wasm32"))]
fn console_logger() -> (ConsoleLogger, log::LevelFilter) {
    let inner = env_logger::Builder::from_default_env().build();
    let level = inner.filter();
    (inner, level)
}

#[cfg(target_arch = "wasm32")]
fn console_logger() -> (ConsoleLogger, log::LevelFilter) {
    // No RUST_LOG in a browser; warnings and errors are what matter in the
    // devtools console, and the UI shows them too.
    let level = log::LevelFilter::Warn;
    (eframe::WebLogger::new(level), level)
}
