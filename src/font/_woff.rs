// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! The tables of a WOFF (1.0) file, each zlib-compressed on its own unless that would not make
//! it smaller, and the sfnt font they rebuild into. Not re-exported.

use super::_inflate::zlib_decompress;
use super::_sfnt::{Tables, be16, be32, is_wanted};
use super::FontError;
use std::borrow::Cow;
use std::cmp::Ordering;

/// A table larger than this is refused instead of inflated: the names, weight and style tables
/// are a few kilobytes.
const MAX_TABLE_SIZE: usize = 16 * 1024 * 1024;

/// The sfnt a whole WOFF file may rebuild into: glyph tables of a large font are megabytes, not
/// gigabytes.
const MAX_SFNT_SIZE: usize = 256 * 1024 * 1024;

/// The data of a table: as stored, or inflated to `size` bytes when `size` is at most `limit`.
fn table(data: &[u8], size: usize, limit: usize) -> Result<Cow<'_, [u8]>, FontError> {
    match data.len().cmp(&size) {
        Ordering::Equal => Ok(Cow::Borrowed(data)),
        Ordering::Less if size <= limit => zlib_decompress(data, size)
            .map(Cow::Owned)
            .ok_or(FontError::InvalidCompressedData),
        _ => Err(FontError::InvalidCompressedData),
    }
}

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
        let table = table(data, size, MAX_TABLE_SIZE)?;
        entries.push((tag, table));
    }
    Ok(Tables { entries })
}

/// The sfnt font a WOFF file was made from: every table inflated, in a table directory the way
/// TrueType and OpenType files have it. The checksums are the ones the WOFF file records.
pub(crate) fn to_sfnt(bytes: &[u8]) -> Result<Vec<u8>, FontError> {
    rebuild(bytes, MAX_SFNT_SIZE)
}

/// [`to_sfnt`] for tables that add up to at most `max` bytes.
fn rebuild(bytes: &[u8], max: usize) -> Result<Vec<u8>, FontError> {
    let header = bytes.get(..44).ok_or(FontError::Truncated)?;
    let count = be16(header, 12);
    let mut tables = Vec::new();
    let mut total = 0usize;
    for index in 0..usize::from(count) {
        let at = 44 + 20 * index;
        let record = bytes.get(at..at + 20).ok_or(FontError::Truncated)?;
        let offset = be32(record, 4) as usize;
        let stored = be32(record, 8) as usize;
        let size = be32(record, 12) as usize;
        total = total.saturating_add(size);
        if total > max {
            return Err(FontError::InvalidCompressedData);
        }
        let data = bytes
            .get(offset..offset.saturating_add(stored))
            .ok_or(FontError::Truncated)?;
        tables.push((&record[..4], be32(record, 16), table(data, size, max)?));
    }

    let entry_selector = count.checked_ilog2().unwrap_or(0);
    let search_range = count
        .checked_ilog2()
        .map_or(0, |power| u16::try_from(16u32 << power).unwrap_or(u16::MAX));
    let mut out = Vec::new();
    out.extend(&header[4..8]);
    out.extend(count.to_be_bytes());
    out.extend(search_range.to_be_bytes());
    out.extend(u16::try_from(entry_selector).unwrap_or(0).to_be_bytes());
    out.extend((count.saturating_mul(16).saturating_sub(search_range)).to_be_bytes());
    let mut offset = 12 + 16 * tables.len();
    for (tag, checksum, data) in &tables {
        out.extend(*tag);
        out.extend(checksum.to_be_bytes());
        out.extend(u32::try_from(offset).unwrap_or(u32::MAX).to_be_bytes());
        out.extend(u32::try_from(data.len()).unwrap_or(u32::MAX).to_be_bytes());
        offset += data.len().next_multiple_of(4);
    }
    for (_, _, data) in &tables {
        out.extend(&**data);
        out.resize(out.len().next_multiple_of(4), 0);
    }
    Ok(out)
}

#[cfg(test)]
#[path = "_woff.test.rs"]
mod tests;
