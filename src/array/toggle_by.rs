// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Like [`toggle`](super::toggle), but elements are the same when `key` gives the same value:
/// toggle objects by id instead of by whole-value equality. Only the first match is removed.
///
/// # Arguments
///
/// - `items` - The current elements.
/// - `item` - The element to add, or whose key to remove.
/// - `key` - Returns the identity to compare.
///
/// # Returns
///
/// A new `Vec`; `items` is not changed.
///
/// # Examples
///
/// ```
/// use helpers4::array::toggle_by;
///
/// let layers = [("a", 1), ("b", 2)];
/// // Same id: removed, whatever the other fields hold.
/// assert_eq!(toggle_by(&layers, ("x", 1), |layer| layer.1), vec![("b", 2)]);
/// assert_eq!(toggle_by(&layers, ("c", 3), |layer| layer.1), vec![("a", 1), ("b", 2), ("c", 3)]);
/// ```
#[must_use]
pub fn toggle_by<T: Clone, K: PartialEq>(
    items: &[T],
    item: T,
    mut key: impl FnMut(&T) -> K,
) -> Vec<T> {
    let wanted = key(&item);
    let mut out = items.to_vec();
    if let Some(index) = out.iter().position(|existing| key(existing) == wanted) {
        out.remove(index);
    } else {
        out.push(item);
    }
    out
}

#[cfg(test)]
#[path = "toggle_by.test.rs"]
mod tests;

#[cfg(test)]
#[path = "toggle_by.spec.rs"]
mod spec;
