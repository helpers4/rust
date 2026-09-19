// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

type Map = ExpiringMap<u32, u32, u64>;

#[test]
fn a_new_map_is_empty() {
    let map = Map::new();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    assert!(Map::default().is_empty());
}

#[test]
fn entries_are_live_until_their_expiry() {
    let mut map = Map::new();
    assert_eq!(map.insert(1, 10, 100, 0), None);
    assert!(!map.is_empty());
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1, 0), Some(&10));
    assert_eq!(map.get(&1, 99), Some(&10));
    assert_eq!(map.get(&1, 100), None);
    assert!(map.contains_key(&1, 99));
    assert!(!map.contains_key(&1, 100));
    assert_eq!(map.get(&2, 0), None);
}

#[test]
fn insert_replaces_a_live_entry_and_returns_the_old_value() {
    let mut map = Map::new();
    map.insert(1, 10, 100, 0);
    assert_eq!(map.insert(1, 11, 200, 50), Some(10));
    assert_eq!(map.get(&1, 150), Some(&11));
}

#[test]
fn insert_over_an_expired_entry_returns_nothing() {
    let mut map = Map::new();
    map.insert(1, 10, 100, 0);
    assert_eq!(map.insert(1, 11, 300, 100), None);
    assert_eq!(map.len(), 1);
}

#[test]
fn insert_if_absent_is_a_replay_check() {
    let mut map = Map::new();
    assert!(map.insert_if_absent(1, 10, 100, 0));
    assert!(!map.insert_if_absent(1, 11, 200, 50));
    assert_eq!(map.get(&1, 60), Some(&10));
    assert!(map.insert_if_absent(1, 12, 300, 100));
    assert_eq!(map.get(&1, 150), Some(&12));
}

#[test]
fn remove_returns_the_value_only_while_live() {
    let mut map = Map::new();
    map.insert(1, 10, 100, 0);
    map.insert(2, 20, 100, 0);
    assert_eq!(map.remove(&1, 50), Some(10));
    assert_eq!(map.remove(&1, 50), None);
    assert_eq!(map.remove(&2, 100), None);
    assert!(map.is_empty());
}

#[test]
fn writes_sweep_expired_entries() {
    let mut map = Map::new();
    map.insert(1, 10, 10, 0);
    map.insert(2, 20, 20, 0);
    map.insert(3, 30, 1000, 0);
    assert_eq!(map.len(), 3);
    map.insert(4, 40, 1000, 25);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&3, 25), Some(&30));
}

#[test]
fn evict_expired_reports_how_many_it_dropped() {
    let mut map = Map::new();
    assert_eq!(map.evict_expired(5), 0);
    map.insert(1, 10, 10, 0);
    map.insert(2, 20, 20, 0);
    assert_eq!(map.evict_expired(9), 0);
    assert_eq!(map.evict_expired(10), 1);
    assert_eq!(map.evict_expired(10), 0);
    assert_eq!(map.evict_expired(500), 1);
    assert!(map.is_empty());
    assert_eq!(map.evict_expired(1000), 0);
}

#[test]
fn an_entry_that_expires_at_once_is_swept_by_the_next_write() {
    let mut map = Map::new();
    map.insert(1, 10, 5, 5);
    assert_eq!(map.get(&1, 5), None);
    map.insert(2, 20, 100, 6);
    assert_eq!(map.len(), 1);
}

#[test]
fn clear_forgets_everything() {
    let mut map = Map::new();
    map.insert(1, 10, 100, 0);
    assert!(!map.is_empty());
    map.clear();
    assert!(map.is_empty());
    assert!(map.insert_if_absent(1, 10, 100, 0));
}

#[test]
fn works_as_an_expiring_set() {
    let mut set: ExpiringSet<u32, u64> = ExpiringSet::new();
    assert!(set.insert_if_absent(7, (), 10, 0));
    assert!(!set.insert_if_absent(7, (), 10, 5));
    assert!(set.contains_key(&7, 9));
}
