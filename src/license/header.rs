// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The two-line source header that carries a copyright notice and an SPDX license identifier.
///
/// ```text
/// Copyright (C) 2025 Jane Doe
/// SPDX-License-Identifier: MIT
/// ```
///
/// The text has no comment marker, so prefix each line for the language of the file
/// (`// `, `# `, ...). The [REUSE](https://reuse.software) specification and most license
/// scanners read exactly these two lines. Nothing is validated: `years` can be `"2025"` or
/// `"2020-2025"`, and `expression` any SPDX expression (see
/// [`Expression`](super::Expression)).
///
/// # Arguments
///
/// - `holder` - The copyright holder, such as a name or an organization.
/// - `years` - The year or range of years of the notice.
/// - `expression` - The SPDX license identifier or expression.
///
/// # Returns
///
/// The two lines, without a trailing newline.
///
/// # Examples
///
/// ```
/// use helpers4::license::header;
///
/// let header = header("Jane Doe", "2025", "MIT OR Apache-2.0");
/// assert_eq!(header, "Copyright (C) 2025 Jane Doe\nSPDX-License-Identifier: MIT OR Apache-2.0");
/// let commented: Vec<String> = header.lines().map(|line| format!("// {line}")).collect();
/// assert_eq!(commented[1], "// SPDX-License-Identifier: MIT OR Apache-2.0");
/// ```
#[must_use]
pub fn header(holder: &str, years: &str, expression: &str) -> String {
    format!("Copyright (C) {years} {holder}\nSPDX-License-Identifier: {expression}")
}

#[cfg(test)]
#[path = "header.test.rs"]
mod tests;
