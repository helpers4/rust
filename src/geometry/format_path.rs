// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::PathCommand;

fn write_number(n: f64, precision: usize) -> String {
    let mut text = format!("{n:.precision$}");
    if text.contains('.') {
        text.truncate(text.trim_end_matches('0').trim_end_matches('.').len());
    }
    if text == "-0" {
        text = "0".to_string();
    }
    text
}

/// Writes path commands back as SVG path data: letters glued to their first number, single
/// spaces elsewhere, numbers rounded to `precision` decimals with the trailing zeros dropped
/// (`-0` is written `0`).
///
/// # Arguments
///
/// - `commands` - The commands, as [`parse_path`](super::parse_path) returns them.
/// - `precision` - Decimals kept per number; `6` is a good default.
///
/// # Returns
///
/// The path data.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::{format_path, PathCommand};
///
/// let d = format_path(
///     &[
///         PathCommand::new('M', vec![0.0, 0.5]),
///         PathCommand::new('L', vec![10.0, 1.0 / 3.0]),
///     ],
///     6,
/// );
/// assert_eq!(d, "M0 0.5 L10 0.333333");
/// ```
#[must_use]
pub fn format_path(commands: &[PathCommand], precision: usize) -> String {
    commands
        .iter()
        .map(|command| {
            let numbers: Vec<String> = command
                .values
                .iter()
                .map(|n| write_number(*n, precision))
                .collect();
            format!("{}{}", command.letter, numbers.join(" "))
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "format_path.test.rs"]
mod tests;
