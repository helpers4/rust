// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{ParsePathError, PathCommand};

/// How many numbers a command takes, `None` for a letter that is not a command.
fn arity(letter: char) -> Option<usize> {
    match letter.to_ascii_lowercase() {
        'm' | 'l' | 't' => Some(2),
        'h' | 'v' => Some(1),
        'c' => Some(6),
        's' | 'q' => Some(4),
        'a' => Some(7),
        'z' => Some(0),
        _ => None,
    }
}

fn skip_separators(bytes: &[u8], from: usize) -> usize {
    from + bytes[from..]
        .iter()
        .take_while(|b| b.is_ascii_whitespace() || **b == b',')
        .count()
}

fn digits(bytes: &[u8], from: usize) -> usize {
    from + bytes[from..]
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .count()
}

/// Reads a number at `i`, returning it with the offset after it.
fn read_number(text: &str, i: usize) -> Option<(f64, usize)> {
    let bytes = text.as_bytes();
    let mut end = i;
    if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
        end += 1;
    }
    let int_end = digits(bytes, end);
    let mut seen = int_end > end;
    end = int_end;
    if end < bytes.len() && bytes[end] == b'.' {
        let frac_end = digits(bytes, end + 1);
        seen |= frac_end > end + 1;
        end = frac_end;
    }
    if !seen {
        return None;
    }
    if end < bytes.len() && (bytes[end] == b'e' || bytes[end] == b'E') {
        let mut exp = end + 1;
        if exp < bytes.len() && (bytes[exp] == b'+' || bytes[exp] == b'-') {
            exp += 1;
        }
        let exp_end = digits(bytes, exp);
        if exp_end > exp {
            end = exp_end;
        }
    }
    text[i..end].parse().ok().map(|n| (n, end))
}

/// Parses SVG path data (the `d` attribute) into commands, as written: the letters keep their
/// case (absolute or relative), `H`/`V`/`S`/`T`/`A` stay what they are, and a command repeated
/// without its letter (`L 1 2 3 4`) becomes one command per set of numbers (`M` repeats as `L`,
/// `m` as `l`).
///
/// It follows the SVG grammar: numbers may be glued (`1-2`, `.5.5`, `1e-3`), separators are
/// optional, and the two arc flags may be written without any separator (`a1 1 0 011 1`).
///
/// # Arguments
///
/// - `d` - The path data.
///
/// # Returns
///
/// The commands, in order. Empty for an empty or blank string.
///
/// # Errors
///
/// A [`ParsePathError`] if the data does not start with a move, has an unknown letter, or misses
/// numbers or flags.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::{parse_path, PathCommand};
///
/// let commands = parse_path("M0 0h10v10z")?;
/// assert_eq!(commands[0], PathCommand::new('M', vec![0.0, 0.0]));
/// assert_eq!(commands[1], PathCommand::new('h', vec![10.0]));
/// assert_eq!(commands.len(), 4);
/// # Ok::<(), helpers4::geometry::ParsePathError>(())
/// ```
pub fn parse_path(d: &str) -> Result<Vec<PathCommand>, ParsePathError> {
    let bytes = d.as_bytes();
    let mut commands: Vec<PathCommand> = Vec::new();
    let mut i = skip_separators(bytes, 0);
    while i < bytes.len() {
        let letter = char::from(bytes[i]);
        let Some(count) = arity(letter) else {
            return Err(ParsePathError::UnknownCommand { index: i });
        };
        if commands.is_empty() && letter != 'M' && letter != 'm' {
            return Err(ParsePathError::MissingMove);
        }
        i += 1;
        let mut current = letter;
        let mut first = true;
        loop {
            let mut values = Vec::with_capacity(count);
            for k in 0..count {
                i = skip_separators(bytes, i);
                if letter.eq_ignore_ascii_case(&'a') && (k == 3 || k == 4) {
                    match bytes.get(i) {
                        Some(b'0') => values.push(0.0),
                        Some(b'1') => values.push(1.0),
                        _ => return Err(ParsePathError::ExpectedFlag { index: i }),
                    }
                    i += 1;
                } else {
                    let (n, end) =
                        read_number(d, i).ok_or(ParsePathError::ExpectedNumber { index: i })?;
                    values.push(n);
                    i = end;
                }
            }
            commands.push(PathCommand::new(current, values));
            if first && (letter == 'M' || letter == 'm') {
                current = if letter == 'M' { 'L' } else { 'l' };
            }
            first = false;
            i = skip_separators(bytes, i);
            if count == 0 || i >= bytes.len() || bytes[i].is_ascii_alphabetic() {
                break;
            }
        }
    }
    Ok(commands)
}

#[cfg(test)]
#[path = "parse_path.test.rs"]
mod tests;

#[cfg(test)]
#[path = "parse_path.spec.rs"]
mod spec;
