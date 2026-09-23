// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::string::squish;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    c.bench_function("string::squish", |b| {
        b.iter(|| squish(black_box("  hello   \n\t world  and   more  space  ")));
    });
}
