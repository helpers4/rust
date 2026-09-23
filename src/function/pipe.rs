// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Combines two functions into one that applies `first` and then `second` to its result:
/// `pipe(first, second)(x)` is `second(first(x))`.
///
/// The same as [`compose`](super::compose) with the arguments in reading order.
///
/// # Arguments
///
/// - `first` - The function applied first.
/// - `second` - The function applied to the result of `first`.
///
/// # Returns
///
/// A function from the input of `first` to the output of `second`.
///
/// # Examples
///
/// ```
/// use helpers4::function::pipe;
///
/// let shout = pipe(|s: &str| s.to_uppercase(), |s: String| s + "!");
/// assert_eq!(shout("hello"), "HELLO!");
/// ```
pub fn pipe<A, B, C>(first: impl Fn(A) -> B, second: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |input| second(first(input))
}

#[cfg(test)]
#[path = "pipe.test.rs"]
mod tests;

#[cfg(test)]
#[path = "pipe.spec.rs"]
mod spec;
