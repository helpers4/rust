// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn a_message_built_from_parts_parses_back_to_them(
        kind in "[a-z]{1,8}",
        scope in proptest::option::of("[a-z-]{1,8}"),
        breaking in any::<bool>(),
        description in "[a-z][a-z ]{0,20}[a-z]",
        body in proptest::option::of("[a-z][a-z ]{0,20}[a-z]"),
    ) {
        let message = format!(
            "{kind}{}{}: {description}{}",
            scope.as_ref().map_or(String::new(), |s| format!("({s})")),
            if breaking { "!" } else { "" },
            body.as_ref().map_or(String::new(), |b| format!("\n\n{b}")),
        );
        let commit = Commit::parse(&message).unwrap();
        prop_assert_eq!(commit.kind(), kind);
        prop_assert_eq!(commit.scope(), scope.as_deref());
        prop_assert_eq!(commit.is_breaking(), breaking);
        prop_assert_eq!(commit.description(), description);
        prop_assert_eq!(commit.body(), body.as_deref());
    }

    #[test]
    fn display_then_parse_gives_the_same_commit(
        kind in "[a-z]{1,8}",
        breaking in any::<bool>(),
        description in "[a-z][a-z ]{0,20}[a-z]",
        footer in proptest::option::of(("[A-Z][a-z]{1,6}", "[a-z0-9]{1,6}")),
    ) {
        let footer_text = footer.as_ref().map_or(String::new(), |(t, v)| format!("\n\n{t}: {v}"));
        let message = format!("{kind}{}: {description}{footer_text}", if breaking { "!" } else { "" });
        let commit = Commit::parse(&message).unwrap();
        prop_assert_eq!(Commit::parse(&commit.to_string()), Ok(commit));
    }

    #[test]
    fn parsing_never_panics(s in "\\PC{0,60}") {
        let _ = Commit::parse(&s);
    }
}
