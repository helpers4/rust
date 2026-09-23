// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{Status, overall};

/// A Markdown summary of a pipeline, ready for a PR comment or a job summary.
///
/// One heading with the overall status (the worst of the jobs, see [`overall`](super::overall)),
/// then a table with one row per job in the order given: its icon, name and status. A `|` in a job
/// name is escaped and line breaks in the title or a name become spaces, so nothing can break the
/// table. There is no trailing newline.
///
/// # Arguments
///
/// - `title` - The heading, such as `"Pull Request Validation"`.
/// - `jobs` - The jobs with their status, as `(name, status)`.
///
/// # Returns
///
/// The Markdown text.
///
/// # Examples
///
/// ```
/// use helpers4::ci::{render_report, Status};
///
/// let report = render_report("Validation", &[("build", Status::Success), ("tests", Status::Failure)]);
/// assert!(report.starts_with("### \u{274c} Validation"));
/// assert!(report.contains("| \u{2705} | build | `success` |"));
/// ```
#[must_use]
pub fn render_report(title: &str, jobs: &[(&str, Status)]) -> String {
    let statuses: Vec<Status> = jobs.iter().map(|(_, status)| *status).collect();
    let clean = |text: &str| text.replace('|', "\\|").replace(['\r', '\n'], " ");
    let mut lines = vec![
        format!(
            "### {} {}",
            overall(&statuses).icon(),
            title.replace(['\r', '\n'], " ")
        ),
        String::new(),
        "| | Job | Status |".to_string(),
        "|:-:|-----|:------:|".to_string(),
    ];
    for (name, status) in jobs {
        lines.push(format!(
            "| {} | {} | `{}` |",
            status.icon(),
            clean(name),
            status.label()
        ));
    }
    lines.join("\n")
}

#[cfg(test)]
#[path = "render_report.test.rs"]
mod tests;

#[cfg(test)]
#[path = "render_report.spec.rs"]
mod spec;
