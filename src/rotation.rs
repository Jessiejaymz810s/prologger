//! File rotation logic for size-based log file rotation.
//!
//! When a log file exceeds the configured size limit, it is rotated
//! by renaming it with a numeric suffix and creating a new file.

use std::fs;
use std::io;
use std::path::PathBuf;

/// Configuration for log file rotation.
#[derive(Debug, Clone)]
pub struct RotationConfig {
    /// Maximum file size in bytes before rotation.
    pub max_size: u64,
    /// Maximum number of rotated files to keep.
    pub max_files: usize,
}

impl RotationConfig {
    /// Creates a new rotation config with the given size limit and file count.
    ///
    /// # Arguments
    ///
    /// * `max_size` - Maximum file size in bytes before triggering rotation.
    /// * `max_files` - Maximum number of backup files to retain.
    ///
    /// # Example
    ///
    /// ```
    /// use prologger::rotation::RotationConfig;
    ///
    /// // Rotate at 10MB, keep 5 backup files
    /// let config = RotationConfig::new(10_000_000, 5);
    /// ```
    pub fn new(max_size: u64, max_files: usize) -> Self {
        Self {
            max_size,
            max_files,
        }
    }

    /// Creates a config that rotates at the given megabyte threshold.
    pub fn megabytes(mb: u64, max_files: usize) -> Self {
        Self::new(mb * 1_000_000, max_files)
    }
}

impl Default for RotationConfig {
    fn default() -> Self {
        // 10 MB, keep 5 files
        Self::new(10_000_000, 5)
    }
}

/// Manages log file rotation.
pub(crate) struct Rotator {
    base_path: PathBuf,
    config: RotationConfig,
}

impl Rotator {
    /// Creates a new rotator for the given file path and configuration.
    pub fn new(path: impl Into<PathBuf>, config: RotationConfig) -> Self {
        Self {
            base_path: path.into(),
            config,
        }
    }

    /// Checks if the current log file needs rotation and performs it if so.
    ///
    /// Returns `Ok(true)` if rotation was performed, `Ok(false)` otherwise.
    pub fn rotate_if_needed(&self) -> io::Result<bool> {
        let metadata = match fs::metadata(&self.base_path) {
            Ok(m) => m,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(false),
            Err(e) => return Err(e),
        };

        if metadata.len() >= self.config.max_size {
            self.rotate()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Performs the rotation by shifting existing files up by one index
    /// and renaming the current file to `.1`.
    fn rotate(&self) -> io::Result<()> {
        // Delete the oldest file if it exists
        let oldest = self.rotated_path(self.config.max_files);
        if oldest.exists() {
            fs::remove_file(&oldest)?;
        }

        // Shift existing rotated files up by one
        for i in (1..self.config.max_files).rev() {
            let from = self.rotated_path(i);
            let to = self.rotated_path(i + 1);
            if from.exists() {
                fs::rename(&from, &to)?;
            }
        }

        // Rename current file to .1
        let first_rotated = self.rotated_path(1);
        if self.base_path.exists() {
            fs::rename(&self.base_path, &first_rotated)?;
        }

        Ok(())
    }

    /// Returns the path for a rotated file with the given index.
    ///
    /// e.g., `app.log` -> `app.log.1`, `app.log.2`, etc.
    fn rotated_path(&self, index: usize) -> PathBuf {
        let mut path = self.base_path.as_os_str().to_owned();
        path.push(format!(".{}", index));
        PathBuf::from(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_rotation_config_defaults() {
        let config = RotationConfig::default();
        assert_eq!(config.max_size, 10_000_000);
        assert_eq!(config.max_files, 5);
    }

    #[test]
    fn test_rotation_config_megabytes() {
        let config = RotationConfig::megabytes(5, 3);
        assert_eq!(config.max_size, 5_000_000);
        assert_eq!(config.max_files, 3);
    }

    #[test]
    fn test_rotate_creates_backup() {
        let dir = std::env::temp_dir().join("prologger_test_rotation");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let log_path = dir.join("test.log");

        // Create a file that exceeds the size limit
        {
            let mut f = fs::File::create(&log_path).unwrap();
            f.write_all(&[b'x'; 100]).unwrap();
        }

        let rotator = Rotator::new(&log_path, RotationConfig::new(50, 3));
        assert!(rotator.rotate_if_needed().unwrap());

        // Original should be gone, .1 should exist
        assert!(!log_path.exists());
        assert!(dir.join("test.log.1").exists());

        // Clean up
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_no_rotation_under_limit() {
        let dir = std::env::temp_dir().join("prologger_test_no_rotation");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let log_path = dir.join("test.log");

        {
            let mut f = fs::File::create(&log_path).unwrap();
            f.write_all(&[b'x'; 10]).unwrap();
        }

        let rotator = Rotator::new(&log_path, RotationConfig::new(100, 3));
        assert!(!rotator.rotate_if_needed().unwrap());
        assert!(log_path.exists());

        let _ = fs::remove_dir_all(&dir);
    }
}
