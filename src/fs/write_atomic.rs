// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Writes `contents` to `path` so that readers see either the old file or the new one, never a
/// half-written one.
///
/// The data goes to a temporary file in the same directory (so the last step stays on one file
/// system), is flushed to disk, and is then renamed over `path`. If anything fails the temporary
/// file is removed and `path` is left as it was. The parent directory must exist.
///
/// # Arguments
///
/// - `path` - The file to create or replace.
/// - `contents` - The bytes to write.
///
/// # Errors
///
/// An [`io::Error`] when `path` has no file name, the temporary file cannot be created or
/// written, or the final rename fails (for instance because `path` is a directory).
///
/// # Examples
///
/// ```
/// use helpers4::fs::write_atomic;
///
/// let path = std::env::temp_dir().join("helpers4-write-atomic-doc.txt");
/// write_atomic(&path, "first")?;
/// write_atomic(&path, "second")?;
/// assert_eq!(std::fs::read_to_string(&path)?, "second");
/// # std::fs::remove_file(&path)?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn write_atomic(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
    write_bytes(path.as_ref(), contents.as_ref())
}

// Non-generic, so every branch is one instantiation to cover, not one per argument type.
fn write_bytes(path: &Path, contents: &[u8]) -> io::Result<()> {
    let name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "path has no file name"))?;
    let mut temp_name = std::ffi::OsString::from(".");
    temp_name.push(name);
    temp_name.push(format!(
        ".{}-{}.tmp",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    let temp = path.with_file_name(temp_name);
    let result = File::create(&temp)
        .and_then(|mut file| file.write_all(contents).and_then(|()| file.sync_all()))
        .and_then(|()| std::fs::rename(&temp, path));
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}

#[cfg(test)]
#[path = "write_atomic.test.rs"]
mod tests;
