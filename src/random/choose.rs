// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rng;

/// Picks one element at random, each with the same probability.
///
/// # Arguments
///
/// - `items` - The elements to pick from.
/// - `rng` - The generator that decides the pick.
///
/// # Returns
///
/// A reference to the chosen element, or `None` when `items` is empty.
///
/// # Examples
///
/// ```
/// use helpers4::random::{choose, Rng};
///
/// let names = ["ana", "bo", "cy"];
/// let pick = choose(&names, &mut Rng::seeded(3));
/// assert!(pick.is_some_and(|name| names.contains(name)));
/// assert_eq!(choose::<u8>(&[], &mut Rng::seeded(3)), None);
/// ```
#[must_use]
pub fn choose<'a, T>(items: &'a [T], rng: &mut Rng) -> Option<&'a T> {
    items.get(rng.index(items.len()))
}

#[cfg(test)]
#[path = "choose.test.rs"]
mod tests;

#[cfg(test)]
#[path = "choose.spec.rs"]
mod spec;
