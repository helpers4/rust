// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::borrow::Cow;

/// Removes the ANSI escape sequences from `text`, leaving what a terminal would display.
///
/// Handles the colors and styles (`ESC [ ... m`) and every other CSI sequence (cursor moves, erase),
/// operating system commands such as window titles and hyperlinks (`ESC ] ... BEL` or
/// `ESC ] ... ESC \`), and the two-character escapes (`ESC c`, `ESC 7`, `ESC ( B`). A lone
/// escape character is dropped, and a sequence cut short at the end of the text is dropped with
/// it. Returns the input borrowed, without allocating, when it has no escape character.
///
/// # Arguments
///
/// - `text` - The text that may contain escape sequences, such as captured command output.
///
/// # Returns
///
/// The text without escape sequences.
///
/// # Examples
///
/// ```
/// use helpers4::ansi::strip;
///
/// assert_eq!(strip("\u{1b}[1;31merror\u{1b}[0m: it broke"), "error: it broke");
/// assert_eq!(strip("plain"), "plain");
/// ```
#[must_use]
pub fn strip(text: &str) -> Cow<'_, str> {
    if !text.contains(ESC) {
        return Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c != ESC {
            out.push(c);
            continue;
        }
        match chars.peek().copied() {
            Some('[') => {
                chars.next();
                // Parameter and intermediate bytes, then one final byte from '@' to '~'.
                for n in chars.by_ref() {
                    if ('@'..='~').contains(&n) {
                        break;
                    }
                }
            }
            Some(']') => {
                chars.next();
                // A string ended by BEL or by ST (ESC \).
                while let Some(n) = chars.next() {
                    if n == '\u{7}' {
                        break;
                    }
                    if n == ESC {
                        if chars.peek() == Some(&'\\') {
                            chars.next();
                        }
                        break;
                    }
                }
            }
            _ => {
                while chars.peek().is_some_and(|n| (' '..='/').contains(n)) {
                    chars.next();
                }
                if chars.peek().is_some_and(|n| ('0'..='~').contains(n)) {
                    chars.next();
                }
            }
        }
    }
    Cow::Owned(out)
}

const ESC: char = '\u{1b}';

#[cfg(test)]
#[path = "strip.test.rs"]
mod tests;

#[cfg(test)]
#[path = "strip.spec.rs"]
mod spec;
