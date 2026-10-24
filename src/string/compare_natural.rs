// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

/// Compares two strings in natural order: runs of ASCII digits are compared by their numeric
/// value, everything else character by character. So `"frame 2"` comes before `"frame 10"`,
/// where plain text order puts `"frame 10"` first.
///
/// Letters compare by code point, so case matters (`"B"` before `"a"`): lowercase both sides
/// first for a case-insensitive order. Numbers of any length are compared exactly, without
/// converting them to an integer, and two numbers of equal value but different zero padding
/// (`"a1"` and `"a01"`) are ordered by padding, fewest zeros first, so only identical strings
/// compare as equal.
///
/// # Arguments
///
/// - `a` - The first string.
/// - `b` - The second string.
///
/// # Returns
///
/// How `a` orders against `b`. Usable as a comparator: `names.sort_by(|a, b| compare_natural(a, b))`.
///
/// # Examples
///
/// ```
/// use helpers4::string::compare_natural;
///
/// let mut frames = ["frame 10", "frame 2", "frame 1"];
/// frames.sort_by(|a, b| compare_natural(a, b));
/// assert_eq!(frames, ["frame 1", "frame 2", "frame 10"]);
/// ```
#[must_use]
pub fn compare_natural(a: &str, b: &str) -> Ordering {
    let (mut left, mut right) = (a.chars().peekable(), b.chars().peekable());
    // Set by numbers of equal value but different padding, used only if the rest is equal.
    let mut padding = Ordering::Equal;
    loop {
        match (left.peek().copied(), right.peek().copied()) {
            (None, None) => return padding,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) if x.is_ascii_digit() && y.is_ascii_digit() => {
                let (ordering, left_zeros, right_zeros) = compare_numbers(&mut left, &mut right);
                if ordering != Ordering::Equal {
                    return ordering;
                }
                if padding == Ordering::Equal {
                    padding = left_zeros.cmp(&right_zeros);
                }
            }
            (Some(x), Some(y)) => {
                if x != y {
                    return x.cmp(&y);
                }
                left.next();
                right.next();
            }
        }
    }
}

/// Consumes one run of digits from each side and compares their values, returning also how many
/// leading zeros each had.
fn compare_numbers(
    left: &mut Peekable<Chars<'_>>,
    right: &mut Peekable<Chars<'_>>,
) -> (Ordering, usize, usize) {
    let (left_zeros, left_digits) = take_number(left);
    let (right_zeros, right_digits) = take_number(right);
    // Without leading zeros, a longer number is larger; of equal length, the first difference wins.
    let ordering = left_digits
        .len()
        .cmp(&right_digits.len())
        .then_with(|| left_digits.cmp(&right_digits));
    (ordering, left_zeros, right_zeros)
}

fn take_number(chars: &mut Peekable<Chars<'_>>) -> (usize, String) {
    let mut zeros = 0;
    while chars.next_if_eq(&'0').is_some() {
        zeros += 1;
    }
    let mut digits = String::new();
    while let Some(digit) = chars.next_if(char::is_ascii_digit) {
        digits.push(digit);
    }
    (zeros, digits)
}

#[cfg(test)]
#[path = "compare_natural.test.rs"]
mod tests;

#[cfg(test)]
#[path = "compare_natural.spec.rs"]
mod spec;
