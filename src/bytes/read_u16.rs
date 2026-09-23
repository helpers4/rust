// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Endian;

/// Reads a `u16` from `bytes` at `offset`, in the given byte order.
///
/// # Arguments
///
/// - `bytes` - The buffer to read from.
/// - `offset` - The index of the first byte to read.
/// - `endian` - The byte order of the value in the buffer.
///
/// # Returns
///
/// The value, or `None` when fewer than 2 bytes remain at `offset`.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::{read_u16, Endian};
///
/// let bytes = [0x12, 0x34];
/// assert_eq!(read_u16(&bytes, 0, Endian::Big), Some(0x1234));
/// assert_eq!(read_u16(&bytes, 0, Endian::Little), Some(0x3412));
/// assert_eq!(read_u16(&bytes, 1, Endian::Big), None);
/// ```
#[must_use]
pub fn read_u16(bytes: &[u8], offset: usize, endian: Endian) -> Option<u16> {
    let end = offset.checked_add(2)?;
    let s = bytes.get(offset..end)?;
    let raw = [s[0], s[1]];
    Some(match endian {
        Endian::Big => u16::from_be_bytes(raw),
        Endian::Little => u16::from_le_bytes(raw),
    })
}

#[cfg(test)]
#[path = "read_u16.test.rs"]
mod tests;

#[cfg(test)]
#[path = "read_u16.spec.rs"]
mod spec;
