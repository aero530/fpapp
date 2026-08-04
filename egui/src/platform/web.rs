//! Browser file handling.
//!
//! The DOM work lives in `file_io.js` — picking a file, writing one back, and
//! deciding between the File System Access API and the `<input type="file">` /
//! download fallback.  This module only spawns those promises and turns their
//! results into `FileEvent`s.

use std::path::Path;

use wasm_bindgen::prelude::*;

use super::{FileEvent, FileIo, SaveMode};

#[wasm_bindgen(module = "/src/platform/file_io.js")]
extern "C" {
    /// Resolves to `{name, text}`, or null when the user cancelled.
    #[wasm_bindgen(catch, js_name = openJsonFile)]
    async fn open_json_file() -> Result<JsValue, JsValue>;

    /// Resolves to `{name}`, or null when the user cancelled.
    #[wasm_bindgen(catch, js_name = saveJsonFile)]
    async fn save_json_file(
        suggested_name: String,
        text: String,
        in_place: bool,
    ) -> Result<JsValue, JsValue>;
}

impl FileIo {
    /// Ask the user for a plan file and read it.
    pub fn open(&self) {
        let io = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match open_json_file().await {
                Ok(result) if is_nothing(&result) => {} // cancelled
                Ok(result) => match (field(&result, "name"), field(&result, "text")) {
                    (Some(name), Some(text)) => io.finish(FileEvent::Opened {
                        path: None,
                        name,
                        text,
                    }),
                    _ => io.finish(FileEvent::Failed(String::from(
                        "Failed to open file: the browser returned no contents.",
                    ))),
                },
                Err(err) => {
                    io.finish(FileEvent::Failed(format!(
                        "Failed to open file: {}",
                        describe(&err)
                    )));
                }
            }
        });
    }

    /// Write `text` back out to the user's machine.  `current` is always `None`
    /// here — the browser keeps the file's location to itself, so `file_io.js`
    /// tracks the handle instead.
    pub fn save(
        &self,
        mode: SaveMode,
        _current: Option<&Path>,
        suggested_name: &str,
        text: String,
    ) {
        let io = self.clone();
        let suggested_name = suggested_name.to_owned();
        let in_place = mode == SaveMode::InPlace;
        wasm_bindgen_futures::spawn_local(async move {
            match save_json_file(suggested_name.clone(), text, in_place).await {
                Ok(result) if is_nothing(&result) => {} // cancelled
                Ok(result) => io.finish(FileEvent::Saved {
                    path: None,
                    name: field(&result, "name").unwrap_or(suggested_name),
                }),
                Err(err) => {
                    io.finish(FileEvent::Failed(format!(
                        "Failed to save file: {}",
                        describe(&err)
                    )));
                }
            }
        });
    }
}

fn is_nothing(value: &JsValue) -> bool {
    value.is_null() || value.is_undefined()
}

/// A string property of a JS object, if it is there.
fn field(value: &JsValue, key: &str) -> Option<String> {
    js_sys::Reflect::get(value, &JsValue::from_str(key))
        .ok()
        .and_then(|value| value.as_string())
}

/// Best-effort message for a JS exception.
fn describe(err: &JsValue) -> String {
    err.as_string()
        .or_else(|| field(err, "message"))
        .unwrap_or_else(|| String::from("the browser reported an unknown error"))
}

/// The calendar year, from the browser's clock.
pub fn current_year() -> u32 {
    js_sys::Date::new_0().get_full_year()
}
