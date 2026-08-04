//! Desktop file handling: modal `rfd` dialogs and direct filesystem access.
//!
//! The work is done by the time these return; results still go through the
//! event queue so the UI has a single code path shared with the web build.

use std::path::Path;

use super::{FileEvent, FileIo, SaveMode};

impl FileIo {
    /// Ask for a plan file and read it.
    pub fn open(&self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .pick_file()
        else {
            return; // cancelled
        };
        match std::fs::read_to_string(&path) {
            Ok(text) => self.finish(FileEvent::Opened {
                name: file_name(&path),
                path: Some(path),
                text,
            }),
            Err(e) => self.finish(FileEvent::Failed(format!("Failed to open file: {}", e))),
        }
    }

    /// Write `text` out, prompting for a location when asked to or when the
    /// plan has no file yet.
    pub fn save(&self, mode: SaveMode, current: Option<&Path>, suggested_name: &str, text: String) {
        let path = match (mode, current) {
            (SaveMode::InPlace, Some(path)) => path.to_path_buf(),
            _ => {
                let Some(path) = rfd::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_file_name(suggested_name)
                    .save_file()
                else {
                    return; // cancelled
                };
                path
            }
        };
        match std::fs::write(&path, text) {
            Ok(()) => self.finish(FileEvent::Saved {
                name: file_name(&path),
                path: Some(path),
            }),
            Err(e) => self.finish(FileEvent::Failed(format!("Failed to save file: {}", e))),
        }
    }
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

/// The calendar year, derived from the system clock.  Average Gregorian year
/// length is close enough here — worst case it is off by a day around New
/// Year, and it only seeds the default `yearStart`.
pub fn current_year() -> u32 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    1970 + (secs / 31_556_952) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file_io() -> FileIo {
        FileIo::new(eframe::egui::Context::default())
    }

    /// "Save" over an existing file needs no dialog, so the whole write path is
    /// testable.
    #[test]
    fn saving_in_place_writes_the_file_and_reports_where() {
        let dir = std::env::temp_dir().join(format!("fpapp-save-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("could not create the temp directory");
        let path = dir.join("plan.json");

        let io = file_io();
        io.save(
            SaveMode::InPlace,
            Some(&path),
            "plan.json",
            String::from("{\"settings\":{}}"),
        );

        assert_eq!(
            std::fs::read_to_string(&path).expect("file was not written"),
            "{\"settings\":{}}"
        );
        match io.next_event() {
            Some(FileEvent::Saved {
                path: Some(saved),
                name,
            }) => {
                assert_eq!(saved, path);
                assert_eq!(name, "plan.json");
            }
            _ => panic!("the save was not reported back to the app"),
        }

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_write_failure_is_reported_rather_than_swallowed() {
        let io = file_io();
        let path = std::env::temp_dir()
            .join("fpapp-no-such-directory")
            .join("plan.json");

        io.save(SaveMode::InPlace, Some(&path), "plan.json", String::new());

        match io.next_event() {
            Some(FileEvent::Failed(message)) => {
                assert!(message.contains("Failed to save file"), "{}", message);
            }
            _ => panic!("a failed write must reach the UI"),
        }
    }
}
