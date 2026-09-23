// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Endian;

/// Reads a `u64` from `bytes` at `offset`, in the given byte order.
///
/// # Arguments
///
/// - `bytes` - The buffer to read from.
/// - `offset` - The index of the first byte to read.
/// - `endian` - The byte order of the value in the buffer.
///
/// # Returns
///
/// The value, or `None` when fewer than 8 bytes remain at `offset`.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::{read_u64, Endian};
///
/// let bytes = [0, 0, 0, 0, 0, 0, 1, 0];
/// assert_eq!(read_u64(&bytes, 0, Endian::Big), Some(256));
/// assert_eq!(read_u64(&bytes, 0, Endian::Little), Some(0x0001_0000_0000_0000));
/// assert_eq!(read_u64(&bytes, 1, Endian::Big), None);
/// ```
#[must_use]
pub fn read_u64(bytes: &[u8], offset: usize, endian: Endian) -> Option<u64> {
    let end = offset.checked_add(8)?;
    let s = bytes.get(offset..end)?;
    let raw = [s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]];
    Some(match endian {
        Endian::Big => u64::from_be_bytes(raw),
        Endian::Little => u64::from_le_bytes(raw),
    })
}

#[cfg(test)]
#[path = "read_u64.test.rs"]
mod tests;

#[cfg(test)]
#[path = "read_u64.spec.rs"]
mod spec;
