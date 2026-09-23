// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// `criterion_group!` and `criterion_main!` generate public, undocumented items.
#![allow(missing_docs, unreachable_pub)]

use criterion::{criterion_group, criterion_main};

#[path = "../src/string/camel_case.bench.rs"]
mod camel_case;
#[path = "../src/string/capitalize.bench.rs"]
mod capitalize;
#[path = "../src/string/constant_case.bench.rs"]
mod constant_case;
#[path = "../src/string/dedent.bench.rs"]
mod dedent;
#[path = "../src/string/escape_html.bench.rs"]
mod escape_html;
#[path = "../src/string/indent.bench.rs"]
mod indent;
#[path = "../src/string/slugify.bench.rs"]
mod slugify;
#[path = "../src/string/snake_case.bench.rs"]
mod snake_case;
#[path = "../src/string/squish.bench.rs"]
mod squish;
#[path = "../src/string/title_case.bench.rs"]
mod title_case;
#[path = "../src/string/unescape_html.bench.rs"]
mod unescape_html;

criterion_group!(
    benches,
    camel_case::bench,
    capitalize::bench,
    constant_case::bench,
    dedent::bench,
    escape_html::bench,
    indent::bench,
    slugify::bench,
    snake_case::bench,
    squish::bench,
    title_case::bench,
    unescape_html::bench,
);
criterion_main!(benches);
