// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Combines two functions into one that applies `inner` first and `outer` to its result:
/// `compose(outer, inner)(x)` is `outer(inner(x))`, like the mathematical `outer ∘ inner`.
///
/// See [`pipe`](super::pipe) for the same thing written in reading order.
///
/// # Arguments
///
/// - `outer` - The function applied last.
/// - `inner` - The function applied first.
///
/// # Returns
///
/// A function from the input of `inner` to the output of `outer`.
///
/// # Examples
///
/// ```
/// use helpers4::function::compose;
///
/// let shout = compose(|s: String| s + "!", |s: &str| s.to_uppercase());
/// assert_eq!(shout("hello"), "HELLO!");
/// ```
pub fn compose<A, B, C>(outer: impl Fn(B) -> C, inner: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |input| outer(inner(input))
}

#[cfg(test)]
#[path = "compose.test.rs"]
mod tests;
