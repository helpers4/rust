// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The most items [`subsets`] accepts: 2^16 = 65 536 subsets.
pub const MAX_SUBSET_ITEMS: usize = 16;

/// Every subset of `items` (the power set), including the empty one and `items` itself.
///
/// Subsets are ordered by the bit pattern that selects them (bit `i` set means `items[i]` is
/// in), so the empty subset comes first and `items` itself last, and the elements of each
/// subset keep their original order. The number of subsets doubles with every item, so more than
/// [`MAX_SUBSET_ITEMS`] items are refused.
///
/// # Arguments
///
/// - `items` - The elements to choose from.
///
/// # Returns
///
/// The `2^n` subsets, or `None` when `items` has more than [`MAX_SUBSET_ITEMS`] elements.
///
/// # Examples
///
/// ```
/// use helpers4::set::subsets;
///
/// assert_eq!(
///     subsets(&["a", "b"]),
///     Some(vec![vec![], vec!["a"], vec!["b"], vec!["a", "b"]])
/// );
/// assert_eq!(subsets(&[0; 17]), None);
/// ```
#[must_use]
pub fn subsets<T: Clone>(items: &[T]) -> Option<Vec<Vec<T>>> {
    if items.len() > MAX_SUBSET_ITEMS {
        return None;
    }
    let count = 1usize << items.len();
    Some(
        (0..count)
            .map(|mask| {
                items
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask >> i & 1 == 1)
                    .map(|(_, item)| item.clone())
                    .collect()
            })
            .collect(),
    )
}

#[cfg(test)]
#[path = "subsets.test.rs"]
mod tests;

#[cfg(test)]
#[path = "subsets.spec.rs"]
mod spec;
