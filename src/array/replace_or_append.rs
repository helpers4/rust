// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Replaces the first element that matches with `item`, or appends `item` when none does: the
/// "upsert into a list" gesture.
///
/// # Arguments
///
/// - `items` - The current elements.
/// - `item` - The replacement, or the new element.
/// - `matches` - Tells which existing element `item` replaces.
///
/// # Returns
///
/// A new `Vec`; `items` is not changed.
///
/// # Examples
///
/// ```
/// use helpers4::array::replace_or_append;
///
/// let users = [(1, "ana"), (2, "bo")];
/// assert_eq!(
///     replace_or_append(&users, (1, "Ana"), |u| u.0 == 1),
///     vec![(1, "Ana"), (2, "bo")]
/// );
/// assert_eq!(
///     replace_or_append(&users, (3, "cy"), |u| u.0 == 3),
///     vec![(1, "ana"), (2, "bo"), (3, "cy")]
/// );
/// ```
#[must_use]
pub fn replace_or_append<T: Clone>(
    items: &[T],
    item: T,
    matches: impl FnMut(&T) -> bool,
) -> Vec<T> {
    let mut out = items.to_vec();
    if let Some(index) = out.iter().position(matches) {
        out[index] = item;
    } else {
        out.push(item);
    }
    out
}

#[cfg(test)]
#[path = "replace_or_append.test.rs"]
mod tests;

#[cfg(test)]
#[path = "replace_or_append.spec.rs"]
mod spec;
