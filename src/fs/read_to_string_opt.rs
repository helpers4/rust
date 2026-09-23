// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::io;
use std::path::Path;

/// Reads a whole file as text, giving `None` instead of an error when it does not exist.
///
/// Every other failure (permissions, a directory, invalid UTF-8) is still an error: only "not
/// found" means "no value".
///
/// # Arguments
///
/// - `path` - The file to read.
///
/// # Errors
///
/// Any [`io::Error`] from [`std::fs::read_to_string`] except [`io::ErrorKind::NotFound`].
///
/// # Examples
///
/// ```
/// use helpers4::fs::read_to_string_opt;
///
/// assert_eq!(read_to_string_opt("/definitely/not/here.txt")?, None);
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn read_to_string_opt(path: impl AsRef<Path>) -> io::Result<Option<String>> {
    read_opt(path.as_ref())
}

// Non-generic, so every branch is one instantiation to cover, not one per path type.
fn read_opt(path: &Path) -> io::Result<Option<String>> {
    match std::fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
#[path = "read_to_string_opt.test.rs"]
mod tests;
