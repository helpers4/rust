// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Path commands rewritten with absolute `M`, `L`, `C`, `Q` and `Z` only, for
//! [`transform_path`](super::transform_path). Not re-exported.

use super::_arc_to_cubics::arc_to_cubics;
use super::PathCommand;

/// The control point mirrored through the current point, or the current point itself without one.
fn reflect(current: (f64, f64), control: Option<(f64, f64)>) -> (f64, f64) {
    control.map_or(current, |c| (2.0 * current.0 - c.0, 2.0 * current.1 - c.1))
}

/// Relative commands become absolute, `H`/`V` become `L`, `S` becomes `C`, `T` becomes `Q` and
/// arcs become cubic curves.
pub(crate) fn normalize_path(commands: &[PathCommand]) -> Vec<PathCommand> {
    let mut out = Vec::new();
    let mut current = (0.0, 0.0);
    let mut start = (0.0, 0.0);
    // The control point the next `S` or `T` reflects, when the previous command was a curve of
    // its kind.
    let mut last_cubic: Option<(f64, f64)> = None;
    let mut last_quad: Option<(f64, f64)> = None;

    for command in commands {
        let v = &command.values;
        let (ox, oy) = if command.letter.is_ascii_lowercase() {
            current
        } else {
            (0.0, 0.0)
        };
        let mut cubic = None;
        let mut quad = None;
        match command.letter.to_ascii_uppercase() {
            'M' => {
                current = (v[0] + ox, v[1] + oy);
                start = current;
                out.push(PathCommand::new('M', vec![current.0, current.1]));
            }
            kind @ ('L' | 'H' | 'V') => {
                let x = if kind == 'V' { current.0 } else { v[0] + ox };
                let y = match kind {
                    'H' => current.1,
                    'V' => v[0] + oy,
                    _ => v[1] + oy,
                };
                current = (x, y);
                out.push(PathCommand::new('L', vec![x, y]));
            }
            kind @ ('C' | 'S') => {
                let (first, i) = if kind == 'S' {
                    (reflect(current, last_cubic), 0)
                } else {
                    ((v[0] + ox, v[1] + oy), 2)
                };
                let second = (v[i] + ox, v[i + 1] + oy);
                current = (v[i + 2] + ox, v[i + 3] + oy);
                cubic = Some(second);
                out.push(PathCommand::new(
                    'C',
                    vec![first.0, first.1, second.0, second.1, current.0, current.1],
                ));
            }
            kind @ ('Q' | 'T') => {
                let (control, i) = if kind == 'T' {
                    (reflect(current, last_quad), 0)
                } else {
                    ((v[0] + ox, v[1] + oy), 2)
                };
                quad = Some(control);
                let end = (v[i] + ox, v[i + 1] + oy);
                out.push(PathCommand::new(
                    'Q',
                    vec![control.0, control.1, end.0, end.1],
                ));
                current = end;
            }
            'A' => {
                let end = (v[5] + ox, v[6] + oy);
                if v[0] == 0.0 || v[1] == 0.0 {
                    out.push(PathCommand::new('L', vec![end.0, end.1]));
                } else if end != current {
                    out.extend(arc_to_cubics(
                        current,
                        (v[0], v[1]),
                        v[2],
                        v[3] != 0.0,
                        v[4] != 0.0,
                        end,
                    ));
                }
                current = end;
            }
            _ => {
                out.push(PathCommand::new('Z', Vec::new()));
                current = start;
            }
        }
        last_cubic = cubic;
        last_quad = quad;
    }
    out
}

#[cfg(test)]
#[path = "_normalize_path.test.rs"]
mod tests;
