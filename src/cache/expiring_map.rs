// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashMap;
use std::collections::hash_map::Entry as MapEntry;
use std::hash::Hash;

/// A map whose entries expire, with the clock passed in by the caller.
///
/// Nothing here reads a clock, so it is trivially testable and works with any timestamp: pass
/// unix seconds (`u64`, e.g. the `exp` of a token), milliseconds, or an `Instant`. An entry is
/// live while `now < expires_at`. Expired entries are dropped lazily: every write sweeps them, so
/// the map only grows with the entries inserted inside one lifetime window (a replay-protection
/// store keyed by token id, a cache of pending challenges, ...).
///
/// The sweep is skipped in constant time while nothing can have expired yet, and is a single pass
/// otherwise. `len` counts entries still stored, expired or not, until the next write sweeps them.
///
/// # Examples
///
/// ```
/// use helpers4::cache::ExpiringMap;
///
/// let mut seen: ExpiringMap<&str, (), u64> = ExpiringMap::new();
/// let now = 1_000;
///
/// // First use of a token id, valid until t = 1_060: accepted.
/// assert!(seen.insert_if_absent("jti-1", (), 1_060, now));
/// // The same id again while it is live: a replay.
/// assert!(!seen.insert_if_absent("jti-1", (), 1_060, now + 10));
/// // Once it has expired it can be used (and remembered) again.
/// assert!(seen.insert_if_absent("jti-1", (), 1_200, 1_060));
/// ```
#[derive(Debug, Clone)]
pub struct ExpiringMap<K, V, T = u64> {
    entries: HashMap<K, Stored<V, T>>,
    /// A lower bound of every stored expiry: while `now` is below it, nothing has expired.
    earliest: Option<T>,
}

#[derive(Debug, Clone)]
struct Stored<V, T> {
    value: V,
    expires_at: T,
}

/// A set whose members expire: an [`ExpiringMap`] without values.
pub type ExpiringSet<K, T = u64> = ExpiringMap<K, (), T>;

impl<K, V, T> ExpiringMap<K, V, T> {
    /// Creates an empty map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            earliest: None,
        }
    }

    /// The number of stored entries, including expired ones that no write has swept yet.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether nothing is stored (see [`len`](Self::len)).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Removes every entry, expired or not.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.earliest = None;
    }
}

impl<K, V, T> Default for ExpiringMap<K, V, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash, V, T: Ord + Copy> ExpiringMap<K, V, T> {
    /// Stores `value` under `key` until `expires_at`, replacing any live entry, and returns the
    /// replaced value. Expired entries are swept first. An `expires_at` that is not after `now`
    /// expires immediately.
    pub fn insert(&mut self, key: K, value: V, expires_at: T, now: T) -> Option<V> {
        self.evict_expired(now);
        self.note_expiry(expires_at);
        self.entries
            .insert(key, Stored { value, expires_at })
            .map(|old| old.value)
    }

    /// Stores `value` under `key` only if there is no live entry for it, and returns whether it
    /// did. This is the replay check: `false` means the key was already seen and is still live.
    /// Expired entries are swept first.
    pub fn insert_if_absent(&mut self, key: K, value: V, expires_at: T, now: T) -> bool {
        self.evict_expired(now);
        match self.entries.entry(key) {
            MapEntry::Occupied(_) => false,
            MapEntry::Vacant(slot) => {
                slot.insert(Stored { value, expires_at });
                self.note_expiry(expires_at);
                true
            }
        }
    }

    /// The live value under `key`, or `None` if absent or expired at `now`.
    #[must_use]
    pub fn get(&self, key: &K, now: T) -> Option<&V> {
        self.entries
            .get(key)
            .filter(|stored| now < stored.expires_at)
            .map(|stored| &stored.value)
    }

    /// Whether there is a live entry under `key` at `now`.
    #[must_use]
    pub fn contains_key(&self, key: &K, now: T) -> bool {
        self.get(key, now).is_some()
    }

    /// Removes `key` and returns its value if it was still live at `now`.
    pub fn remove(&mut self, key: &K, now: T) -> Option<V> {
        self.entries
            .remove(key)
            .filter(|stored| now < stored.expires_at)
            .map(|stored| stored.value)
    }

    /// Drops every entry that has expired at `now` and returns how many. Writes do this already;
    /// call it directly to release memory while nothing is being written.
    pub fn evict_expired(&mut self, now: T) -> usize {
        match self.earliest {
            Some(earliest) if now >= earliest => {}
            _ => return 0,
        }
        let before = self.entries.len();
        self.entries.retain(|_, stored| now < stored.expires_at);
        self.earliest = self.entries.values().map(|stored| stored.expires_at).min();
        before - self.entries.len()
    }

    fn note_expiry(&mut self, expires_at: T) {
        self.earliest = Some(
            self.earliest
                .map_or(expires_at, |earliest| earliest.min(expires_at)),
        );
    }
}

#[cfg(test)]
#[path = "expiring_map.test.rs"]
mod tests;

#[cfg(test)]
#[path = "expiring_map.spec.rs"]
mod spec;
