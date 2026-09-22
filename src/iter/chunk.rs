// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Splits `iter` into consecutive chunks of `size` items, the last one possibly shorter.
///
/// Unlike [`slice::chunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks),
/// this consumes any `IntoIterator`, not just a slice, and owns the items instead of borrowing
/// them, so it also works on a lazily-generated or single-use iterator. A `size` of `0` produces
/// no chunks at all, since a non-empty chunk cannot hold zero items.
///
/// # Arguments
///
/// - `iter` - The items to split.
/// - `size` - How many items go in each chunk.
///
/// # Returns
///
/// The chunks, in order.
///
/// # Examples
///
/// ```
/// use helpers4::iter::chunk;
///
/// assert_eq!(chunk(1..=5, 2), vec![vec![1, 2], vec![3, 4], vec![5]]);
/// assert_eq!(chunk(Vec::<i32>::new(), 3), Vec::<Vec<i32>>::new());
/// assert_eq!(chunk(1..=3, 0), Vec::<Vec<i32>>::new());
/// ```
pub fn chunk<I: IntoIterator>(iter: I, size: usize) -> Vec<Vec<I::Item>> {
    if size == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut current = Vec::with_capacity(size);
    for item in iter {
        current.push(item);
        if current.len() == size {
            out.push(std::mem::replace(&mut current, Vec::with_capacity(size)));
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

#[cfg(test)]
#[path = "chunk.test.rs"]
mod tests;

#[cfg(test)]
#[path = "chunk.spec.rs"]
mod spec;
