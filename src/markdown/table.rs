// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A GitHub-flavored Markdown table.
///
/// `headers` gives the columns; each row is padded with empty cells or cut to that many. Cells
/// are cleaned so they cannot break the table: `|` becomes `\|` and line breaks become spaces.
/// Columns are padded to a common width (at least 3, for the `---` separator) so the source
/// lines up, measured in characters. The result has one line per row and ends with a newline;
/// no headers give an empty string. Cell text is not otherwise escaped: use
/// [`escape`](super::escape) on text you do not control.
///
/// # Arguments
///
/// - `headers` - The column titles.
/// - `rows` - The rows, each a list of cells.
///
/// # Returns
///
/// The table.
///
/// # Examples
///
/// ```
/// use helpers4::markdown::table;
///
/// let out = table(&["Name", "Stars"], &[vec!["typescript", "1"], vec!["rust", "0"]]);
/// assert_eq!(
///     out,
///     "| Name       | Stars |\n| ---------- | ----- |\n| typescript | 1     |\n| rust       | 0     |\n"
/// );
/// ```
#[must_use]
pub fn table<H: AsRef<str>, C: AsRef<str>>(headers: &[H], rows: &[Vec<C>]) -> String {
    let headers: Vec<String> = headers.iter().map(|h| clean(h.as_ref())).collect();
    let rows: Vec<Vec<String>> = rows
        .iter()
        .map(|row| row.iter().map(|cell| clean(cell.as_ref())).collect())
        .collect();
    render(&headers, &rows)
}

// Non-generic core: the generic wrapper above has no branches of its own.
fn render(headers: &[String], rows: &[Vec<String>]) -> String {
    let columns = headers.len();
    if columns == 0 {
        return String::new();
    }
    let cell = |row: &[String], column: usize| row.get(column).cloned().unwrap_or_default();
    let widths: Vec<usize> = (0..columns)
        .map(|column| {
            rows.iter()
                .map(|row| cell(row, column).chars().count())
                .chain([headers[column].chars().count(), 3])
                .max()
                .unwrap_or(3)
        })
        .collect();
    let line = |cells: Vec<String>| {
        let padded: Vec<String> = cells
            .iter()
            .zip(&widths)
            .map(|(text, width)| format!("{text:<width$}"))
            .collect();
        format!("| {} |\n", padded.join(" | "))
    };
    let mut out = line(headers.to_vec());
    out.push_str(&line(widths.iter().map(|w| "-".repeat(*w)).collect()));
    for row in rows {
        out.push_str(&line(
            (0..columns).map(|column| cell(row, column)).collect(),
        ));
    }
    out
}

/// Makes a cell safe inside a table row.
fn clean(cell: &str) -> String {
    cell.replace('|', "\\|").replace(['\r', '\n'], " ")
}

#[cfg(test)]
#[path = "table.test.rs"]
mod tests;

#[cfg(test)]
#[path = "table.spec.rs"]
mod spec;
