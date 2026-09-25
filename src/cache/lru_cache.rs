// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashMap;
use std::hash::Hash;

/// A map bounded to at most `capacity` entries: once full, inserting a new key evicts the least
/// recently used one.
///
/// "Used" means read with [`get`](Self::get) or [`get_or_insert_with`](Self::get_or_insert_with),
/// or written with [`insert`](Self::insert); [`contains_key`](Self::contains_key) does not count,
/// so checking for a key without reading it never changes what gets evicted next.
///
/// Unlike [`ExpiringMap`](super::ExpiringMap), entries never expire on their own: eviction is
/// only ever "the entry nobody has touched in the longest time", driven purely by capacity.
///
/// A capacity of `0` is a documented degenerate case, not an error: the cache never stores
/// anything, and every [`insert`](Self::insert) is immediately evicted.
///
/// # Examples
///
/// ```
/// use helpers4::cache::LruCache;
///
/// let mut cache: LruCache<&str, i32> = LruCache::new(2);
/// cache.insert("a", 1);
/// cache.insert("b", 2);
/// cache.get(&"a"); // touch "a": "b" is now the least recently used
/// cache.insert("c", 3); // evicts "b", not "a"
/// assert_eq!(cache.get(&"a"), Some(&1));
/// assert_eq!(cache.get(&"b"), None);
/// assert_eq!(cache.get(&"c"), Some(&3));
/// ```
#[derive(Debug, Clone)]
pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    /// Least recently used first, most recently used last. Kept in lockstep with `map`.
    order: Vec<K>,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    /// Creates an empty cache holding at most `capacity` entries.
    ///
    /// # Arguments
    ///
    /// - `capacity` - The most entries the cache holds before it starts evicting.
    ///
    /// # Returns
    ///
    /// An empty cache.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            order: Vec::new(),
        }
    }

    /// Marks `key` as just used, moving it to the most-recently-used end of the eviction order.
    fn touch(&mut self, key: &K) {
        if let Some(index) = self.order.iter().position(|k| k == key) {
            let key = self.order.remove(index);
            self.order.push(key);
        }
    }

    /// The live value for `key`, marking it as just used.
    ///
    /// # Arguments
    ///
    /// - `key` - The key to look up.
    ///
    /// # Returns
    ///
    /// The value for `key`, or `None` when it is not present.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.touch(key);
        }
        self.map.get(key)
    }

    /// Inserts `value` under `key`, marking it as just used.
    ///
    /// Evicts the least recently used entry first when `key` is new and the cache is already at
    /// capacity.
    ///
    /// # Arguments
    ///
    /// - `key` - The key to insert or update.
    /// - `value` - The value to store.
    ///
    /// # Returns
    ///
    /// The previous value stored under `key`, if any.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.capacity == 0 {
            return None;
        }
        // `order` always holds exactly the keys `map` does (every method keeps them in
        // lockstep), so reaching capacity means `order` is non-empty here too.
        if !self.map.contains_key(&key) && self.map.len() >= self.capacity {
            let lru = self.order.remove(0);
            self.map.remove(&lru);
        }
        self.touch(&key);
        if !self.order.contains(&key) {
            self.order.push(key.clone());
        }
        self.map.insert(key, value)
    }

    /// Whether `key` has a live entry, without affecting eviction order.
    ///
    /// # Arguments
    ///
    /// - `key` - The key to check.
    ///
    /// # Returns
    ///
    /// `true` when `key` has a live entry.
    #[must_use]
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// How many entries are currently stored.
    ///
    /// # Returns
    ///
    /// The number of live entries, at most [`capacity`](Self::capacity).
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Whether the cache holds no entries.
    ///
    /// # Returns
    ///
    /// `true` when [`len`](Self::len) is `0`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// The capacity this cache was created with.
    ///
    /// # Returns
    ///
    /// The most entries the cache holds before it starts evicting.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Removes every entry.
    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }
}

impl<K: Eq + Hash + Clone, V: Clone> LruCache<K, V> {
    /// Returns the value for `key`, computing and storing it with `f` first if absent.
    ///
    /// Bounded, single-key equivalent of [`Memoize`](crate::function::Memoize): the same key
    /// never recomputes while it stays in the cache, but an old key can be evicted to make room.
    /// With a capacity of `0` the freshly computed value is still returned, but is not retained
    /// (nothing is, see [`new`](Self::new)): the next call recomputes it.
    ///
    /// # Arguments
    ///
    /// - `key` - The key to look up.
    /// - `f` - Computes the value to store when `key` is absent. Not called when `key` is
    ///   already present.
    ///
    /// # Returns
    ///
    /// The value for `key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::cache::LruCache;
    ///
    /// let mut calls = 0;
    /// let mut cache: LruCache<u32, u32> = LruCache::new(4);
    /// let square = |cache: &mut LruCache<u32, u32>, n: u32, calls: &mut i32| {
    ///     cache.get_or_insert_with(n, || {
    ///         *calls += 1;
    ///         n * n
    ///     })
    /// };
    /// assert_eq!(square(&mut cache, 7, &mut calls), 49);
    /// assert_eq!(square(&mut cache, 7, &mut calls), 49); // served from the cache
    /// assert_eq!(calls, 1);
    /// ```
    pub fn get_or_insert_with(&mut self, key: K, f: impl FnOnce() -> V) -> V {
        if let Some(value) = self.get(&key) {
            return value.clone();
        }
        let value = f();
        self.insert(key, value.clone());
        value
    }
}

#[cfg(test)]
#[path = "lru_cache.test.rs"]
mod tests;

#[cfg(test)]
#[path = "lru_cache.spec.rs"]
mod spec;
