// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// One command of SVG path data: its letter (uppercase is absolute, lowercase relative to the
/// current point) and its numbers, as written in the `d` attribute.
///
/// | Letter | Command | Numbers |
/// | --- | --- | --- |
/// | `M` `L` `T` | move, line, smooth quadratic | `x y` |
/// | `H` `V` | horizontal, vertical line | `x` or `y` |
/// | `C` | cubic Bézier | `x1 y1 x2 y2 x y` |
/// | `S` `Q` | smooth cubic, quadratic | `x2 y2 x y` or `x1 y1 x y` |
/// | `A` | elliptical arc | `rx ry rotation large-arc sweep x y` |
/// | `Z` | close the subpath | none |
///
/// # Examples
///
/// ```
/// use helpers4::geometry::PathCommand;
///
/// let line = PathCommand::new('L', vec![10.0, 5.0]);
/// assert_eq!(line.letter, 'L');
/// assert_eq!(line.values, [10.0, 5.0]);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PathCommand {
    /// The command letter.
    pub letter: char,
    /// The numbers that follow it.
    pub values: Vec<f64>,
}

impl PathCommand {
    /// Builds a command.
    ///
    /// # Arguments
    ///
    /// - `letter` - The command letter.
    /// - `values` - Its numbers.
    ///
    /// # Returns
    ///
    /// The command.
    #[must_use]
    pub const fn new(letter: char, values: Vec<f64>) -> Self {
        Self { letter, values }
    }
}
