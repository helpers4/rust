// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Compares two byte slices without stopping at the first difference.
///
/// The loop visits every byte and combines the differences with `|`, so the time it takes does
/// not depend on where the slices differ, only on their length (a length mismatch returns at
/// once, so the length is not secret). This is a best effort: Rust and LLVM give no hard
/// guarantee that a compiler will not turn it back into an early exit, so for cryptographic
/// code that needs one, use a dedicated crate such as `subtle`.
///
/// # Arguments
///
/// - `a` - The first slice.
/// - `b` - The second slice.
///
/// # Returns
///
/// `true` when both slices have the same length and the same content.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::constant_time_eq;
///
/// assert!(constant_time_eq(b"secret", b"secret"));
/// assert!(!constant_time_eq(b"secret", b"secreT"));
/// assert!(!constant_time_eq(b"secret", b"secre"));
/// ```
#[must_use]
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b) {
        diff |= x ^ y;
    }
    std::hint::black_box(diff) == 0
}

#[cfg(test)]
#[path = "constant_time_eq.test.rs"]
mod tests;

#[cfg(test)]
#[path = "constant_time_eq.spec.rs"]
mod spec;
