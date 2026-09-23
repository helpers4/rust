// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::str::FromStr;

/// The outcome of a CI job or step.
///
/// # Examples
///
/// ```
/// use helpers4::ci::Status;
///
/// assert_eq!(Status::from_conclusion("failure"), Some(Status::Failure));
/// assert_eq!(Status::Success.icon(), "\u{2705}");
/// assert_eq!(Status::Success.label(), "success");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    /// Finished without errors.
    Success,
    /// Finished with an error, or timed out.
    Failure,
    /// Stopped before it finished.
    Cancelled,
    /// Not run.
    Skipped,
    /// Queued or still running.
    Pending,
}

impl Status {
    /// Reads the conclusion names CI services use, ignoring case.
    ///
    /// `success`, `passed` and `pass` are [`Status::Success`]; `failure`, `failed`, `error` and
    /// `timed_out` are [`Status::Failure`]; `cancelled` and `canceled` are [`Status::Cancelled`];
    /// `skipped` is [`Status::Skipped`]; `pending`, `queued`, `in_progress` and `running` are
    /// [`Status::Pending`].
    ///
    /// # Arguments
    ///
    /// - `conclusion` - The name, such as `"success"` or `"cancelled"`.
    ///
    /// # Returns
    ///
    /// The status, or `None` for a name that is not one of those.
    #[must_use]
    pub fn from_conclusion(conclusion: &str) -> Option<Self> {
        Some(match conclusion.to_ascii_lowercase().as_str() {
            "success" | "passed" | "pass" => Self::Success,
            "failure" | "failed" | "error" | "timed_out" => Self::Failure,
            "cancelled" | "canceled" => Self::Cancelled,
            "skipped" => Self::Skipped,
            "pending" | "queued" | "in_progress" | "running" => Self::Pending,
            _ => return None,
        })
    }

    /// An emoji for the status, for a report or a PR comment.
    ///
    /// # Returns
    ///
    /// `\u{2705}` for success, `\u{274c}` for failure, `\u{1f6ab}` for cancelled, `\u{23ed}\u{fe0f}` for skipped
    /// and `\u{23f3}` for pending.
    #[must_use]
    pub fn icon(self) -> &'static str {
        match self {
            Self::Success => "\u{2705}",
            Self::Failure => "\u{274c}",
            Self::Cancelled => "\u{1f6ab}",
            Self::Skipped => "\u{23ed}\u{fe0f}",
            Self::Pending => "\u{23f3}",
        }
    }

    /// The lower-case name of the status.
    ///
    /// # Returns
    ///
    /// `"success"`, `"failure"`, `"cancelled"`, `"skipped"` or `"pending"`.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
            Self::Cancelled => "cancelled",
            Self::Skipped => "skipped",
            Self::Pending => "pending",
        }
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(conclusion: &str) -> Result<Self, Self::Err> {
        Self::from_conclusion(conclusion).ok_or(())
    }
}

#[cfg(test)]
#[path = "status.test.rs"]
mod tests;
