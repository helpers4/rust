// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::string::unescape_html;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    c.bench_function("string::unescape_html", |b| {
        b.iter(|| {
            unescape_html(black_box(
                "&lt;p class=&quot;a&quot;&gt;Tom &amp; Jerry&#39;s &#x41;&lt;/p&gt;",
            ))
        });
    });
}
