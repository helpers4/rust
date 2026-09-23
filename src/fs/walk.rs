// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Every file below `dir`, at any depth, sorted by path.
///
/// Directories themselves are not listed (an empty one contributes nothing), and symbolic links
/// are listed as entries but not followed, so a link to a parent directory cannot make the walk
/// loop.
///
/// # Arguments
///
/// - `dir` - The directory to walk.
///
/// # Errors
///
/// An [`io::Error`] when `dir`, or a directory below it, cannot be read.
///
/// # Examples
///
/// ```
/// use helpers4::fs::{walk, write_atomic};
///
/// let root = std::env::temp_dir().join("helpers4-walk-doc");
/// std::fs::create_dir_all(root.join("sub"))?;
/// write_atomic(root.join("a.txt"), "a")?;
/// write_atomic(root.join("sub").join("b.txt"), "b")?;
/// assert_eq!(walk(&root)?, vec![root.join("a.txt"), root.join("sub").join("b.txt")]);
/// # std::fs::remove_dir_all(&root)?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn walk(dir: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    walk_dir(dir.as_ref())
}

// Non-generic, so every branch is one instantiation to cover, not one per path type.
fn walk_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut pending = vec![dir.to_path_buf()];
    while let Some(current) = pending.pop() {
        let entries: io::Result<Vec<_>> = fs::read_dir(&current).and_then(|read| {
            read.map(|entry| entry.and_then(|e| e.file_type().map(|kind| (e.path(), kind))))
                .collect()
        });
        for (path, kind) in entries? {
            if kind.is_dir() {
                pending.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

#[cfg(test)]
#[path = "walk.test.rs"]
mod tests;
