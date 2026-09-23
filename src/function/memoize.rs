// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashMap;
use std::hash::Hash;

/// A function whose results are remembered: calling it again with the same argument returns
/// the stored result instead of computing it again.
///
/// The wrapped function receives the argument by reference and must be deterministic (and free
/// of side effects you rely on), since it runs only once per distinct argument. There is no
/// size limit: every distinct argument keeps its result until [`clear`](Self::clear) is called,
/// so do not feed it unbounded input.
///
/// # Examples
///
/// ```
/// use helpers4::function::Memoize;
///
/// let mut square = Memoize::new(|n: &u64| n * n);
/// assert_eq!(square.call(12), 144);
/// assert_eq!(square.call(12), 144); // served from memory
/// assert_eq!(square.len(), 1);
/// ```
pub struct Memoize<A, R, F> {
    func: F,
    cache: HashMap<A, R>,
}

impl<A: Eq + Hash + Clone, R: Clone, F: FnMut(&A) -> R> Memoize<A, R, F> {
    /// Wraps `func`.
    ///
    /// # Arguments
    ///
    /// - `func` - The function whose results to remember.
    ///
    /// # Returns
    ///
    /// A memoized version of `func` with an empty memory.
    #[must_use]
    pub fn new(func: F) -> Self {
        Self {
            func,
            cache: HashMap::new(),
        }
    }

    /// Calls the function with `arg`, or returns the remembered result for that argument.
    ///
    /// # Arguments
    ///
    /// - `arg` - The argument to call the function with.
    ///
    /// # Returns
    ///
    /// The result for `arg`, computed at most once until [`clear`](Self::clear).
    pub fn call(&mut self, arg: A) -> R {
        if let Some(result) = self.cache.get(&arg) {
            return result.clone();
        }
        let result = (self.func)(&arg);
        self.cache.insert(arg, result.clone());
        result
    }

    /// The number of distinct arguments whose result is remembered.
    ///
    /// # Returns
    ///
    /// The number of stored results.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Whether no result is remembered yet.
    ///
    /// # Returns
    ///
    /// `true` when nothing is stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Forgets every remembered result.
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
#[path = "memoize.test.rs"]
mod tests;

#[cfg(test)]
#[path = "memoize.spec.rs"]
mod spec;
