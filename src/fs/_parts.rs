// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Lexical path normalization shared by `normalize` and `is_within`. Not re-exported.

use std::path::{Component, Path};

/// The components of `path` with `.` removed and `..` folded into the component before it.
///
/// A `..` with nothing to fold into stays in a relative path and is dropped after a root (you
/// cannot go above `/`). Nothing touches the file system, so symbolic links are not resolved.
pub(crate) fn parts(path: &Path) -> Vec<Component<'_>> {
    let mut out: Vec<Component<'_>> = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => match out.last() {
                Some(Component::Normal(_)) => {
                    out.pop();
                }
                Some(Component::RootDir | Component::Prefix(_)) => {}
                _ => out.push(component),
            },
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
#[path = "_parts.test.rs"]
mod tests;
