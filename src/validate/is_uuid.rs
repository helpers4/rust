// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Checks whether `s` is a UUID in its canonical `8-4-4-4-12` hyphenated hexadecimal form.
///
/// Case-insensitive. The variant and version digits are not checked, so this accepts any RFC
/// 9562 UUID (v1 through v8) as well as the all-zero nil UUID; it only checks the shape.
///
/// # Arguments
///
/// - `s` - The text to check.
///
/// # Returns
///
/// `true` when `s` has the shape of a UUID.
///
/// # Examples
///
/// ```
/// use helpers4::validate::is_uuid;
///
/// assert!(is_uuid("550e8400-e29b-41d4-a716-446655440000"));
/// assert!(is_uuid("550E8400-E29B-41D4-A716-446655440000"));
/// assert!(!is_uuid("550e8400-e29b-41d4-a716-44665544000")); // one digit short
/// assert!(!is_uuid("not-a-uuid"));
/// ```
#[must_use]
pub fn is_uuid(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 36 {
        return false;
    }
    bytes.iter().enumerate().all(|(i, &b)| {
        if matches!(i, 8 | 13 | 18 | 23) {
            b == b'-'
        } else {
            b.is_ascii_hexdigit()
        }
    })
}

#[cfg(test)]
#[path = "is_uuid.test.rs"]
mod tests;

#[cfg(test)]
#[path = "is_uuid.spec.rs"]
mod spec;
