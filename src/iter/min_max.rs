// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Returns the smallest and largest item of `iter` in one pass, or `None` when it is empty.
///
/// Equivalent to calling
/// [`.min()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min) and
/// [`.max()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max) separately, but
/// only iterates once, so it also works on an iterator that can only be consumed a single time.
/// Comparisons follow `T`'s [`PartialOrd`]: with `f64`, a `NaN` is neither smaller nor larger
/// than anything, exactly as `<` and `>` say, so a `NaN` that is never replaced (for instance the
/// very first item) stays in the result.
///
/// # Arguments
///
/// - `iter` - The items to scan.
///
/// # Returns
///
/// `(min, max)`, or `None` when `iter` is empty.
///
/// # Examples
///
/// ```
/// use helpers4::iter::min_max;
///
/// assert_eq!(min_max(1..=5), Some((1, 5)));
/// assert_eq!(min_max([3, 1, 4, 1, 5]), Some((1, 5)));
/// assert_eq!(min_max(Vec::<i32>::new()), None);
/// ```
pub fn min_max<T: PartialOrd + Copy>(iter: impl IntoIterator<Item = T>) -> Option<(T, T)> {
    let mut it = iter.into_iter();
    let first = it.next()?;
    let mut min = first;
    let mut max = first;
    for item in it {
        if item < min {
            min = item;
        }
        if item > max {
            max = item;
        }
    }
    Some((min, max))
}

#[cfg(test)]
#[path = "min_max.test.rs"]
mod tests;

#[cfg(test)]
#[path = "min_max.spec.rs"]
mod spec;
