// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Removes `item` from `items` if it is there, adds it at the end if it is not: the usual
/// "toggle a selection" gesture. Only the first occurrence is removed.
///
/// # Arguments
///
/// - `items` - The current elements.
/// - `item` - The element to toggle, compared with `==`.
///
/// # Returns
///
/// A new `Vec`; `items` is not changed.
///
/// # Examples
///
/// ```
/// use helpers4::array::toggle;
///
/// assert_eq!(toggle(&[1, 2, 3], 2), vec![1, 3]);
/// assert_eq!(toggle(&[1, 3], 2), vec![1, 3, 2]);
/// ```
#[must_use]
pub fn toggle<T: Clone + PartialEq>(items: &[T], item: T) -> Vec<T> {
    let mut out = items.to_vec();
    if let Some(index) = out.iter().position(|existing| *existing == item) {
        out.remove(index);
    } else {
        out.push(item);
    }
    out
}

#[cfg(test)]
#[path = "toggle.test.rs"]
mod tests;

#[cfg(test)]
#[path = "toggle.spec.rs"]
mod spec;
