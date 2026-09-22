// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Returns every pair `(x, y)` with `x` from `a` and `y` from `b`, in row-major order.
///
/// For more than two inputs, nest the calls.
///
/// # Arguments
///
/// - `a` - The first slice.
/// - `b` - The second slice.
///
/// # Examples
///
/// ```
/// use helpers4::array::cartesian_product;
///
/// assert_eq!(
///     cartesian_product(&[1, 2], &['a', 'b']),
///     vec![(1, 'a'), (1, 'b'), (2, 'a'), (2, 'b')]
/// );
/// ```
pub fn cartesian_product<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter()
        .flat_map(|x| b.iter().map(move |y| (x.clone(), y.clone())))
        .collect()
}

#[cfg(test)]
#[path = "cartesian_product.test.rs"]
mod tests;

#[cfg(test)]
#[path = "cartesian_product.spec.rs"]
mod spec;
