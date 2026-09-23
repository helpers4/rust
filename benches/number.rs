// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// `criterion_group!` and `criterion_main!` generate public, undocumented items.
#![allow(missing_docs, unreachable_pub)]

use criterion::{criterion_group, criterion_main};

#[path = "../src/number/gcd.bench.rs"]
mod gcd;
#[path = "../src/number/mean.bench.rs"]
mod mean;
#[path = "../src/number/median.bench.rs"]
mod median;

criterion_group!(benches, gcd::bench, mean::bench, median::bench,);
criterion_main!(benches);
