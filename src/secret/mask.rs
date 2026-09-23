// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Hides a secret but for its last few characters, such as `"****************7890"`.
///
/// Every character except the last `visible` becomes `*`. To be safe with short secrets, at most a
/// quarter of the characters are ever shown, whatever `visible` asks for, so an 8-character
/// password shows at most 2. The length of the result equals the length of the secret in characters.
///
/// # Arguments
///
/// - `secret` - The value to mask.
/// - `visible` - How many trailing characters to keep, at most a quarter of the secret.
///
/// # Returns
///
/// The masked value.
///
/// # Examples
///
/// ```
/// use helpers4::secret::mask;
///
/// assert_eq!(mask("sk-abcdef1234567890", 4), "***************7890");
/// assert_eq!(mask("abc", 4), "***"); // too short to show anything
/// ```
#[must_use]
pub fn mask(secret: &str, visible: usize) -> String {
    let length = secret.chars().count();
    let shown = visible.min(length / 4);
    let hidden = length - shown;
    let mut out = "*".repeat(hidden);
    out.extend(secret.chars().skip(hidden));
    out
}

#[cfg(test)]
#[path = "mask.test.rs"]
mod tests;

#[cfg(test)]
#[path = "mask.spec.rs"]
mod spec;
