// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn get_on_missing_key_is_none() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    assert_eq!(cache.get(&"missing"), None);
}

#[test]
fn insert_new_key_returns_none_and_grows() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    assert_eq!(cache.insert("a", 1), None);
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&"a"), Some(&1));
}

#[test]
fn insert_existing_key_returns_previous_value_without_growing() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    cache.insert("a", 1);
    assert_eq!(cache.insert("a", 2), Some(1));
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&"a"), Some(&2));
}

#[test]
fn insert_past_capacity_evicts_least_recently_used() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    cache.insert("a", 1);
    cache.insert("b", 2);
    cache.insert("c", 3); // "a" was never touched: it goes
    assert_eq!(cache.get(&"a"), None);
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.get(&"c"), Some(&3));
    assert_eq!(cache.len(), 2);
}

#[test]
fn get_refreshes_recency_so_it_survives_eviction() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    cache.insert("a", 1);
    cache.insert("b", 2);
    cache.get(&"a"); // "a" is now the most recently used; "b" is the least
    cache.insert("c", 3);
    assert_eq!(cache.get(&"a"), Some(&1));
    assert_eq!(cache.get(&"b"), None);
    assert_eq!(cache.get(&"c"), Some(&3));
}

#[test]
fn contains_key_does_not_affect_eviction_order() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    cache.insert("a", 1);
    cache.insert("b", 2);
    assert!(cache.contains_key(&"a")); // must not count as a use
    cache.insert("c", 3);
    assert_eq!(cache.get(&"a"), None); // still evicted despite the check above
    assert!(!cache.contains_key(&"missing"));
}

#[test]
fn zero_capacity_never_stores_anything() {
    let mut cache: LruCache<&str, i32> = LruCache::new(0);
    assert_eq!(cache.insert("a", 1), None);
    assert_eq!(cache.get(&"a"), None);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

#[test]
fn capacity_reports_what_it_was_created_with() {
    let cache: LruCache<&str, i32> = LruCache::new(5);
    assert_eq!(cache.capacity(), 5);
}

#[test]
fn is_empty_reflects_len() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    assert!(cache.is_empty());
    cache.insert("a", 1);
    assert!(!cache.is_empty());
}

#[test]
fn clear_removes_every_entry() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    cache.insert("a", 1);
    cache.insert("b", 2);
    cache.clear();
    assert!(cache.is_empty());
    assert_eq!(cache.get(&"a"), None);
}

// One closure literal (one generic instantiation of `get_or_insert_with`, since llvm-cov tracks
// each instantiation on its own) called from a loop so the *same* instantiation takes both the
// "absent, call f" and "present, skip f" branch, instead of two textually-distinct closures each
// only ever taking one of the two.
#[test]
fn get_or_insert_with_computes_once_per_key() {
    let mut calls = 0;
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    for _ in 0..2 {
        assert_eq!(
            cache.get_or_insert_with("a", || {
                calls += 1;
                1
            }),
            1
        );
    }
    assert_eq!(calls, 1);
}

#[test]
fn get_or_insert_with_zero_capacity_recomputes_every_time() {
    let mut calls = 0;
    let mut cache: LruCache<&str, i32> = LruCache::new(0);
    for _ in 0..2 {
        assert_eq!(
            cache.get_or_insert_with("a", || {
                calls += 1;
                1
            }),
            1
        );
    }
    assert_eq!(calls, 2);
}

#[test]
fn get_or_insert_with_evicts_like_any_other_insert() {
    let mut cache: LruCache<&str, i32> = LruCache::new(1);
    for key in ["a", "a", "b"] {
        cache.get_or_insert_with(key, || 1);
    }
    assert_eq!(cache.get(&"a"), None);
    assert_eq!(cache.get(&"b"), Some(&1));
}

#[test]
fn debug_and_clone_are_derived() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    cache.insert("a", 1);
    let cloned = cache.clone();
    assert_eq!(format!("{cloned:?}"), format!("{cache:?}"));
}
