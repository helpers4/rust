// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn parse(messages: &[&str]) -> Vec<Commit> {
    messages.iter().map(|m| Commit::parse(m).unwrap()).collect()
}

#[test]
fn takes_the_largest_bump() {
    assert_eq!(
        bump_for(&parse(&["docs: a", "fix: b", "feat: c"])),
        Bump::Minor
    );
    assert_eq!(bump_for(&parse(&["docs: a", "fix: b"])), Bump::Patch);
    assert_eq!(bump_for(&parse(&["feat: a", "fix!: b"])), Bump::Major);
}

#[test]
fn commits_that_do_not_affect_the_version_give_none() {
    assert_eq!(bump_for(&parse(&["docs: a", "chore: b"])), Bump::None);
}

#[test]
fn no_commits_give_none() {
    assert_eq!(bump_for(&[]), Bump::None);
}
