// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! The tables of a WOFF (1.0) file, each zlib-compressed on its own unless that would not make
//! it smaller. Not re-exported.

use super::_inflate::zlib_decompress;
use super::_sfnt::{Tables, be16, be32, is_wanted};
use super::FontError;
use std::borrow::Cow;

/// A table larger than this is refused instead of inflated: the names, weight and style tables
/// are a few kilobytes.
const MAX_TABLE_SIZE: usize = 16 * 1024 * 1024;

pub(crate) fn load(bytes: &[u8]) -> Result<Tables<'_>, FontError> {
    let header = bytes.get(..44).ok_or(FontError::Truncated)?;
    let count = usize::from(be16(header, 12));
    let mut entries = Vec::new();
    for index in 0..count {
        let at = 44 + 20 * index;
        let record = bytes.get(at..at + 20).ok_or(FontError::Truncated)?;
        let tag = [record[0], record[1], record[2], record[3]];
        if !is_wanted(tag) {
            continue;
        }
        let offset = be32(record, 4) as usize;
        let stored = be32(record, 8) as usize;
        let size = be32(record, 12) as usize;
        let data = bytes
            .get(offset..offset.saturating_add(stored))
            .ok_or(FontError::Truncated)?;
        let table = if stored == size {
            Cow::Borrowed(data)
        } else if stored < size && size <= MAX_TABLE_SIZE {
            Cow::Owned(zlib_decompress(data, size).ok_or(FontError::InvalidCompressedData)?)
        } else {
            return Err(FontError::InvalidCompressedData);
        };
        entries.push((tag, table));
    }
    Ok(Tables { entries })
}

#[cfg(test)]
#[path = "_woff.test.rs"]
mod tests;
