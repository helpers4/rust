// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Implementation shared by public modules that must each stand alone as a Cargo feature
//! (`cargo test --no-default-features --features <module>` has to pass on its own for every
//! one of them). Not part of the public API: every consumer wraps this in its own public type.
//!
//! Gated on `any(...)` of its consumers' features, together: with no gate at all it would be
//! dead code (and fail clippy's `-D warnings`) for any feature combination that includes
//! neither; gated on a single feature it would not compile for the other one alone.

#[cfg(any(feature = "duration", feature = "env"))]
pub(crate) mod duration_grammar {
    use std::time::Duration;

    /// Mirrors `duration::ParseDurationError` one variant at a time (see that type's docs);
    /// kept crate-private and separate so this module carries no public API of its own.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) enum Error {
        Empty,
        ExpectedNumber { index: usize },
        MissingUnit { index: usize },
        UnknownUnit { index: usize },
        Overflow,
    }

    /// Parses a human-written duration such as `"1h30m"`, `"2d"` or `"500ms"`. See
    /// `duration::parse`, the public wrapper around this, for the accepted syntax.
    pub(crate) fn parse(input: &str) -> Result<Duration, Error> {
        let bytes = input.as_bytes();
        let mut total = Duration::ZERO;
        let mut i = 0;
        let mut any = false;
        loop {
            i = run_end(bytes, i, u8::is_ascii_whitespace);
            if i == bytes.len() {
                break;
            }
            let digits_start = i;
            i = run_end(bytes, i, u8::is_ascii_digit);
            if i == digits_start {
                return Err(Error::ExpectedNumber { index: i });
            }
            let amount: u64 = input[digits_start..i]
                .parse()
                .map_err(|_| Error::Overflow)?;
            let unit_start = i;
            i = run_end(bytes, i, u8::is_ascii_alphabetic);
            if i == unit_start {
                return Err(Error::MissingUnit { index: i });
            }
            let piece = match &input[unit_start..i] {
                "ms" => Some(Duration::from_millis(amount)),
                "s" => Some(Duration::from_secs(amount)),
                "m" => amount.checked_mul(60).map(Duration::from_secs),
                "h" => amount.checked_mul(3600).map(Duration::from_secs),
                "d" => amount.checked_mul(86_400).map(Duration::from_secs),
                "w" => amount.checked_mul(604_800).map(Duration::from_secs),
                _ => return Err(Error::UnknownUnit { index: unit_start }),
            };
            total = piece
                .and_then(|piece| total.checked_add(piece))
                .ok_or(Error::Overflow)?;
            any = true;
        }
        if any { Ok(total) } else { Err(Error::Empty) }
    }

    /// Index just past the run of bytes, starting at `from`, that satisfy `keep`.
    fn run_end(bytes: &[u8], from: usize, keep: fn(&u8) -> bool) -> usize {
        from + bytes[from..].iter().take_while(|byte| keep(byte)).count()
    }
}

/// Number formatting for the CSS output of `color` and `animation`.
#[cfg(any(feature = "color", feature = "animation"))]
pub(crate) mod css_number {
    /// A number for a CSS value: rounded to `decimals` places, without trailing zeros (`12.0` is
    /// `"12"`, `0.50` is `"0.5"`) and without a negative zero, including when a small negative
    /// value rounds to zero.
    pub(crate) fn trimmed(value: f64, decimals: usize) -> String {
        let text = format!("{value:.decimals$}");
        let text = if text.contains('.') {
            text.trim_end_matches('0').trim_end_matches('.')
        } else {
            &text
        };
        // A small negative value rounds to "-0", which CSS reads but nobody means.
        if text == "-0" {
            "0".to_string()
        } else {
            text.to_string()
        }
    }
}

#[cfg(all(test, any(feature = "color", feature = "animation")))]
#[path = "internal.test.rs"]
mod tests;
