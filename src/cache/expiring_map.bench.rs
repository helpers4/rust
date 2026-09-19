// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::cache::ExpiringMap;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    c.bench_function("cache::ExpiringMap/insert_if_absent+get (1000 live)", |b| {
        let mut map: ExpiringMap<u32, u32, u64> = ExpiringMap::new();
        for key in 0..1000 {
            map.insert(key, key, 1_000_000, 0);
        }
        let mut now = 0u64;
        b.iter(|| {
            now += 1;
            let key = black_box(u32::try_from(now % 1000).unwrap_or(0));
            let seen = !map.insert_if_absent(key, key, 1_000_000, now);
            black_box((seen, map.get(&key, now)));
        });
    });
    c.bench_function("cache::ExpiringMap/sweep 1000 expired", |b| {
        b.iter_batched(
            || {
                let mut map: ExpiringMap<u32, u32, u64> = ExpiringMap::new();
                for key in 0..1000 {
                    map.insert(key, key, 10, 0);
                }
                map
            },
            |mut map| black_box(map.evict_expired(black_box(10))),
            criterion::BatchSize::SmallInput,
        );
    });
}
