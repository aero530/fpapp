//! Platform glue: file dialogs, reading/writing the plan, and the clock.
//!
//! The native build touches the filesystem directly and its dialogs are modal.
//! The browser build cannot do either: a page only ever receives a file through
//! a picker, and writes go back out through the File System Access API (or a
//! download, in browsers that lack it) — both asynchronous.
//!
//! To keep the UI free of `cfg` branches, both platforms report results through
//! the same queue.  The UI asks for an operation and forgets about it;
//! `FpApp::apply_file_events` drains completed operations once per frame.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::rc::Rc;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::current_year;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::current_year;

/// A finished file operation.
pub enum FileEvent {
    Opened {
        /// Where the file came from, when the platform can say — always `None`
        /// in the browser, which never reveals paths to the page.
        path: Option<PathBuf>,
        name: String,
        text: String,
    },
    Saved {
        path: Option<PathBuf>,
        name: String,
    },
    Failed(String),
}

/// Where a save should go.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SaveMode {
    /// Overwrite the file the plan came from, prompting only if there is no
    /// such file yet (the "Save" button).
    InPlace,
    /// Always ask the user where to put it (the "Save As..." button).
    Prompt,
}

/// Handle the UI uses to request file operations and collect their results.
#[derive(Clone)]
pub struct FileIo {
    ctx: eframe::egui::Context,
    events: Rc<RefCell<VecDeque<FileEvent>>>,
}

impl FileIo {
    pub fn new(ctx: eframe::egui::Context) -> Self {
        Self {
            ctx,
            events: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    /// The oldest operation that has finished since the last call, if any.
    pub fn next_event(&self) -> Option<FileEvent> {
        self.events.borrow_mut().pop_front()
    }

    /// Stand in for a dialog the tests cannot open.
    #[cfg(test)]
    pub fn inject(&self, event: FileEvent) {
        self.finish(event);
    }

    /// Record a finished operation for the next frame to pick up.
    fn finish(&self, event: FileEvent) {
        self.events.borrow_mut().push_back(event);
        // On the web this runs from a promise callback, outside the paint loop:
        // without a repaint request nothing would wake the app up to notice.
        self.ctx.request_repaint();
    }
}
