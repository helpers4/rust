// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// `criterion_group!` and `criterion_main!` generate public, undocumented items.
#![allow(missing_docs, unreachable_pub)]

use criterion::{criterion_group, criterion_main};

#[path = "../src/net/is_public_ip.bench.rs"]
mod is_public_ip;

criterion_group!(benches, is_public_ip::bench);
criterion_main!(benches);
