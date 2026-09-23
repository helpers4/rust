// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn a_built_url_parses_and_displays_back(
        scheme in "[a-z][a-z0-9+.-]{0,6}",
        host in "[a-z0-9-]{1,10}(\\.[a-z]{2,5}){0,2}",
        port in proptest::option::of(0u16..),
        path in "(/[a-zA-Z0-9._~%-]{0,6}){0,4}",
        query in proptest::option::of("[a-z0-9=&]{0,10}"),
        fragment in proptest::option::of("[a-z0-9]{0,6}"),
    ) {
        let text = format!(
            "{scheme}://{host}{}{path}{}{}",
            port.map_or(String::new(), |p| format!(":{p}")),
            query.as_ref().map_or(String::new(), |q| format!("?{q}")),
            fragment.as_ref().map_or(String::new(), |f| format!("#{f}")),
        );
        let parsed = Url::parse(&text).unwrap();
        prop_assert_eq!(parsed.to_string(), text);
        prop_assert_eq!(parsed.host(), Some(host.as_str()));
        prop_assert_eq!(parsed.port(), port);
    }

    #[test]
    fn joining_an_absolute_reference_gives_that_reference(reference in "[a-z]{1,5}://[a-z]{1,8}/[a-z]{0,8}") {
        let base = Url::parse("http://base.example/dir/page").unwrap();
        prop_assert_eq!(base.join(&reference).unwrap(), Url::parse(&reference).unwrap());
    }

    #[test]
    fn joining_never_leaves_dot_segments(reference in "([a-z]{1,3}|\\.|\\.\\.)(/([a-z]{1,3}|\\.|\\.\\.)){0,5}") {
        let base = Url::parse("http://h/a/b/c").unwrap();
        let joined = base.join(&reference).unwrap();
        prop_assert!(joined.path().split('/').all(|s| s != "." && s != ".."));
    }

    #[test]
    fn parsing_never_panics(s in "\\PC{0,30}") {
        let _ = Url::parse(&s);
        let _ = Url::parse("http://h/a").unwrap().join(&s);
    }
}
