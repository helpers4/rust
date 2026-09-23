// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The emoji the helpers4 commit convention puts after the colon for a commit type, such as `✨`
/// for `feat` and `🐛` for `fix`.
///
/// The type is matched ignoring case. These are the primary emoji of the convention (see
/// `commit-convention.json` in the `.dev` repository), one per standard type: `feat`, `fix`,
/// `docs`, `refactor`, `test`, `chore`, `perf`, `style`, `ci`, `build` and `revert`.
///
/// # Arguments
///
/// - `kind` - The commit type, such as `"feat"`.
///
/// # Returns
///
/// The emoji, or `None` for a type the convention does not define.
///
/// # Examples
///
/// ```
/// use helpers4::commit::emoji_for;
///
/// assert_eq!(emoji_for("feat"), Some("\u{2728}"));
/// assert_eq!(emoji_for("FIX"), Some("\u{1f41b}"));
/// assert_eq!(emoji_for("wip"), None);
/// ```
#[must_use]
pub fn emoji_for(kind: &str) -> Option<&'static str> {
    Some(match kind.to_ascii_lowercase().as_str() {
        "feat" => "\u{2728}",
        "fix" => "\u{1f41b}",
        "docs" => "\u{1f4dd}",
        "refactor" => "\u{267b}\u{fe0f}",
        "test" => "\u{2705}",
        "chore" => "\u{1f527}",
        "perf" => "\u{26a1}\u{fe0f}",
        "style" => "\u{1f484}",
        "ci" => "\u{1f477}",
        "build" => "\u{1f4e6}\u{fe0f}",
        "revert" => "\u{23ea}\u{fe0f}",
        _ => return None,
    })
}

#[cfg(test)]
#[path = "emoji_for.test.rs"]
mod tests;
