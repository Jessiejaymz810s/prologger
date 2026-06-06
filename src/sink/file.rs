//! File sink — writes log output to a file with optional rotation.

use super::Sink;
use crate::formatter::Format;
use crate::rotation::{RotationConfig, Rotator};
use log::Record;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

/// A sink that writes formatted log output to a file.
///
/// Supports optional size-based file rotation.
pub struct FileSink {
    path: PathBuf,
    file: Mutex<File>,
    rotator: Option<Rotator>,
}

impl FileSink {
    /// Creates a new file sink that writes to the given path.
    ///
    /// The file is created if it doesn't exist, or appended to if it does.
    pub fn new(path: impl Into<PathBuf>) -> std::io::Result<Self> {
        let path = path.into();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;

        Ok(Self {
            path,
            file: Mutex::new(file),
            rotator: None,
        })
    }

    /// Creates a new file sink with rotation enabled.
    pub fn with_rotation(
        path: impl Into<PathBuf>,
        config: RotationConfig,
    ) -> std::io::Result<Self> {
        let path = path.into();

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;

        let rotator = Rotator::new(path.clone(), config);

        Ok(Self {
            path,
            file: Mutex::new(file),
            rotator: Some(rotator),
        })
    }

    /// Attempts to rotate the log file if needed, reopening the file handle.
    fn try_rotate(&self) {
        if let Some(ref rotator) = self.rotator {
            match rotator.rotate_if_needed() {
                Ok(true) => {
                    // Reopen the file after rotation
                    if let Ok(new_file) = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&self.path)
                    {
                        if let Ok(mut guard) = self.file.lock() {
                            *guard = new_file;
                        }
                    }
                }
                Ok(false) => {} // No rotation needed
                Err(e) => {
                    eprintln!("[prologger] rotation error: {}", e);
                }
            }
        }
    }
}

// Manual Debug implementation since File doesn't implement Debug
impl std::fmt::Debug for FileSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileSink")
            .field("path", &self.path)
            .finish()
    }
}

impl Sink for FileSink {
    fn write(&self, record: &Record, formatter: &dyn Format) {
        self.try_rotate();

        let formatted = formatter.format(record);
        if let Ok(mut file) = self.file.lock() {
            let _ = file.write_all(formatted.as_bytes());
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
    }
}
