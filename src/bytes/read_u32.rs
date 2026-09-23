// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Endian;

/// Reads a `u32` from `bytes` at `offset`, in the given byte order.
///
/// # Arguments
///
/// - `bytes` - The buffer to read from.
/// - `offset` - The index of the first byte to read.
/// - `endian` - The byte order of the value in the buffer.
///
/// # Returns
///
/// The value, or `None` when fewer than 4 bytes remain at `offset`.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::{read_u32, Endian};
///
/// let bytes = [0x12, 0x34, 0x56, 0x78];
/// assert_eq!(read_u32(&bytes, 0, Endian::Big), Some(0x1234_5678));
/// assert_eq!(read_u32(&bytes, 0, Endian::Little), Some(0x7856_3412));
/// assert_eq!(read_u32(&bytes, 1, Endian::Big), None);
/// ```
#[must_use]
pub fn read_u32(bytes: &[u8], offset: usize, endian: Endian) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let s = bytes.get(offset..end)?;
    let raw = [s[0], s[1], s[2], s[3]];
    Some(match endian {
        Endian::Big => u32::from_be_bytes(raw),
        Endian::Little => u32::from_le_bytes(raw),
    })
}

#[cfg(test)]
#[path = "read_u32.test.rs"]
mod tests;

#[cfg(test)]
#[path = "read_u32.spec.rs"]
mod spec;
