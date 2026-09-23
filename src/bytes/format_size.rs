// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

/// Formats a number of bytes with a binary unit and at most one decimal, such as `"1.5 KiB"`.
///
/// Units are powers of 1024 (`KiB`, `MiB`, ... `EiB`) and the value is rounded to the nearest
/// tenth, so `1023.96 KiB` reads `1 MiB` rather than `1024 KiB`. A `.0` is dropped. Only integer
/// arithmetic is used, so the result never depends on floating-point rounding.
/// [`parse_size`](super::parse_size) reads it back.
///
/// # Arguments
///
/// - `bytes` - The size to format.
///
/// # Returns
///
/// The formatted size.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::format_size;
///
/// assert_eq!(format_size(512), "512 B");
/// assert_eq!(format_size(1536), "1.5 KiB");
/// assert_eq!(format_size(5 * 1024 * 1024), "5 MiB");
/// assert_eq!(format_size(1_048_575), "1 MiB");
/// ```
#[must_use]
pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{bytes} B");
    }
    let bytes = u128::from(bytes);
    // A u64 is below 1024^7, so `index` stops at 6 (EiB) without needing a bound, and at that
    // unit `tenths` is at most 160, so it never rounds up to a unit that does not exist.
    let mut index = 1;
    let mut unit = 1024u128;
    while bytes >= unit * 1024 {
        unit *= 1024;
        index += 1;
    }
    let mut tenths = (bytes * 10 + unit / 2) / unit;
    if tenths >= 10_240 {
        unit *= 1024;
        index += 1;
        tenths = (bytes * 10 + unit / 2) / unit;
    }
    let (whole, fraction) = (tenths / 10, tenths % 10);
    if fraction == 0 {
        format!("{whole} {}", UNITS[index])
    } else {
        format!("{whole}.{fraction} {}", UNITS[index])
    }
}

#[cfg(test)]
#[path = "format_size.test.rs"]
mod tests;

#[cfg(test)]
#[path = "format_size.spec.rs"]
mod spec;
