// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::{criterion_group, criterion_main};

#[path = "../src/string/camel_case.bench.rs"]
mod camel_case;
#[path = "../src/string/capitalize.bench.rs"]
mod capitalize;
#[path = "../src/string/dedent.bench.rs"]
mod dedent;
#[path = "../src/string/escape_html.bench.rs"]
mod escape_html;
#[path = "../src/string/slugify.bench.rs"]
mod slugify;
#[path = "../src/string/snake_case.bench.rs"]
mod snake_case;

criterion_group!(
    benches,
    camel_case::bench,
    capitalize::bench,
    dedent::bench,
    escape_html::bench,
    slugify::bench,
    snake_case::bench,
);
criterion_main!(benches);
