// +++++++++++++++++++++++++++++++++++++++++++++++++++
// Using

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::mem;
use std::ops;

// +++++++++++++++++++++++++++++++++++++++++++++++++++
// Traits

pub trait SortedMapKey: Debug {
    fn key_cmp(&self, other: &Self) -> Ordering;
}

// +++++++++++++++++++++++++++++++++++++++++++++++++++
// Definition

/**
 * A faster and more powerful version of a sorted map as BTreeMap.
 * Added functions for floor and ceil entries.
 * All types used as keys, must implement the SortedMapKey trait to provide correct order function
 */
#[derive(Debug, Default, Clone)]
pub struct SortedMap<K, V>
where
    K: SortedMapKey,
{
    entries: Vec<(K, V)>,
}

#[derive(Debug, Default, Clone)]
pub struct SortedSet<K>
where
    K: SortedMapKey,
{
    entries: Vec<K>,
}

// +++++++++++++++++++++++++++++++++++++++++++++++++++
// Implementation

#[allow(dead_code)]
impl<K, V> SortedMap<K, V>
where
    K: SortedMapKey,
{
    /**
     * Create a new empty SortedMap
     */
    pub fn new() -> SortedMap<K, V> {
        SortedMap { entries: Vec::new() }
    }

    /**
     * Create a new empty SortedMap with given capacity to avoid reallocation
     */
    pub fn with_capacity(capacity: usize) -> SortedMap<K, V> {
        SortedMap {
            entries: Vec::with_capacity(capacity),
        }
    }

    /**
     * Insert value for given key (natural oder) to this map. If key already exists, value is overriten and old
     * value is returned. If key is new, new entry with given key and value is created.
     */
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(&key)) {
            Ok(index) => Some(mem::replace(&mut self.entries[index].1, value)),
            Err(index) => {
                self.entries.insert(index, (key, value));
                None
            }
        }
    }

    /**
     * Remove key from this SortedMap. Returns old value if key exists, otherwise None.
     */
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(&key)) {
            Ok(index) => Some(self.entries.remove(index).1),
            _ => None,
        }
    }

    /**
     * Check if this map contains a value for given key
     */
    pub fn contains(&self, key: &K) -> bool {
        self.entries.binary_search_by(|(k, _)| k.key_cmp(key)).is_ok()
    }

    /**
     * Get value at given key, panics if key does not exist.
     */
    pub fn get(&self, key: &K) -> &V {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(key)) {
            Ok(index) => &self.entries[index].1,
            _ => panic!("Key does not exist: {:?}", key),
        }
    }

    /**
     * Get value at given key, or None if key does not exist.
     */
    pub fn try_get(&self, key: &K) -> Option<&V> {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(key)) {
            Ok(index) => Some(&self.entries[index].1),
            _ => None,
        }
    }

    /**
     *  Get value at given key, or insert a new value for given key if key does not exist.
     */
    pub fn get_or_insert<F>(&mut self, key: &K, mut func: F) -> &V
    where
        F: FnMut() -> V,
        K: Copy,
    {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(key)) {
            Ok(index) => &self.entries[index].1,
            Err(index) => {
                let value = (func)();
                self.entries.insert(index, (*key, value));
                &self.entries[index].1
            }
        }
    }

    /**
     * Get mutable value at given key, panics if key does not exist.
     */
    pub fn get_mut(&mut self, key: &K) -> &mut V {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(key)) {
            Ok(index) => &mut self.entries[index].1,
            _ => panic!("Key does not exist: {:?}", key),
        }
    }

    /**
     * Get mutable value at given key, or None if key does not exist.
     */
    pub fn try_get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(key)) {
            Ok(index) => Some(&mut self.entries[index].1),
            _ => None,
        }
    }

    /**
     * Get value reference for given key, or next lower key-value if key does not exist.
     * If there is no lower key, None is returned.
     */
    pub fn floor(&self, key: &K) -> Option<&V> {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(key)) {
            Ok(index) => Some(&self.entries[index].1),
            Err(index) => {
                if index > 0 {
                    Some(&self.entries[index - 1].1)
                } else {
                    None
                }
            }
        }
    }

    /**
     * Get mutable value reference for given key, or next lower key-value if key does not exist.
     * If there is no lower key, None is returned.
     */
    pub fn floor_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(key)) {
            Ok(index) => Some(&mut self.entries[index].1),
            Err(index) => {
                if index > 0 {
                    Some(&mut self.entries[index - 1].1)
                } else {
                    None
                }
            }
        }
    }

    /**
     * Get value reference for  given key, or next higher key-value if key does not exist.
     * If there is no higher key, None is returned.
     */
    pub fn ceil(&self, key: &K) -> Option<&V> {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(key)) {
            Ok(index) => Some(&self.entries[index].1),
            Err(index) => {
                if index < self.entries.len() {
                    Some(&self.entries[index].1)
                } else {
                    None
                }
            }
        }
    }

    /**
     * Get value reference for  given key, or next higher key-value if key does not exist.
     * If there is no higher key, None is returned.
     */
    pub fn ceil_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.entries.binary_search_by(|(k, _)| k.key_cmp(key)) {
            Ok(index) => Some(&mut self.entries[index].1),
            Err(index) => {
                if index < self.entries.len() {
                    Some(&mut self.entries[index].1)
                } else {
                    None
                }
            }
        }
    }

    /**
     * Get slice of entries containing key-value pairs where all keys are greater than or equal to key_min and less than key_max (sorted by key)
     */
    pub fn sub(&self, key_min: &K, key_max: &K) -> &[(K, V)] {
        assert!(key_min.key_cmp(key_max) != Ordering::Greater);
        match (self.entries.binary_search_by(|(k, _)| k.key_cmp(key_min)), self.entries.binary_search_by(|(k, _)| k.key_cmp(key_max))) {
            (Ok(index_min), Ok(index_max)) => &self.entries[index_min..index_max],
            (Ok(index_min), Err(index_max)) => &self.entries[index_min..index_max],
            (Err(index_min), Ok(index_max)) => &self.entries[index_min..index_max],
            (Err(index_min), Err(index_max)) => &self.entries[index_min..index_max],
        }
    }

    /**
     * Swap keys and values of this instance to create a new inversed map
     */
    pub fn inverse(self) -> SortedMap<V, K>
    where
        V: SortedMapKey + Send,
        K: Send,
    {
        self.entries.into_iter().map(|(k, v)| (v, k)).collect()
    }

    /**
     * Extend this set by another set of the same keys. Values will moved from other to this
     */
    pub fn extend(&mut self, other: SortedMap<K, V>) {
        self.entries.extend(other.into_iter());
        self.sort();
    }

    /**
     * Makes all entries a sorted set, sorting and removing duplicates
     */
    fn sort(&mut self) {
        self.entries.sort_by(|(k0, _), (k1, _)| k0.key_cmp(k1));
        self.entries.dedup_by(|(k0, _), (k1, _)| k1.key_cmp(k0) == Ordering::Equal);
    }

    /**
     * Get length of this map
     */
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /**
     * Check if this map is empty
     */
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /**
     * Get first value in this Map (with lowest key), panics if map is empty
     */
    #[inline]
    pub fn first(&self) -> &V {
        self.entries.first().map(|(_, v)| v).expect("Map is empty")
    }

    /**
     * Get Option<first value> in this Map (with lowest key), or None is map is empty
     */
    #[inline]
    pub fn try_first(&self) -> Option<&V> {
        self.entries.first().map(|(_, v)| v)
    }

    /**
     * Get last value in this Map (with highest key), panics if map is empty
     */
    #[inline]
    pub fn last(&self) -> &V {
        self.entries.last().map(|(_, v)| v).expect("Map is empty")
    }

    /**
     * Get last value in this Map (with highest key)
     */
    #[inline]
    pub fn try_last(&self) -> Option<&V> {
        self.entries.last().map(|(_, v)| v)
    }

    /**
     * Get first key in this Map, panics if map is empty
     */
    #[inline]
    pub fn first_key(&self) -> &K {
        self.entries.first().map(|(k, _)| k).expect("Map is empty")
    }

    /**
     * Get first key in this Map, or None if map is empty
     */
    #[inline]
    pub fn try_first_key(&self) -> Option<&K> {
        self.entries.first().map(|(k, _)| k)
    }

    /**
     * Get last key in this Map, panics if map is empty
     */
    #[inline]
    pub fn last_key(&self) -> &K {
        self.entries.last().map(|(k, _)| k).expect("Map is empty")
    }

    /**
     * Get last key in this Map, or None if map is empty
     */
    #[inline]
    pub fn try_last_key(&self) -> Option<&K> {
        self.entries.last().map(|(k, _)| k)
    }

    /**
     * Get an iterator over all entry-tuples (ordered by key)
     */
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.entries.iter()
    }

    /**
     * Get an iterator over all keys (ordered by key)
     */
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.entries.iter().map(|(k, _)| k)
    }

    /**
     * Get an iterator over all values (ordered by key)
     */
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.iter().map(|(_, v)| v)
    }
}

#[allow(dead_code)]
impl<K> SortedSet<K>
where
    K: SortedMapKey,
{
    /**
     * Get value at given key, or None if key does not exist.
     */
    pub fn contains(&self, key: &K) -> bool {
        self.entries.binary_search_by(|k| k.key_cmp(key)).is_ok()
    }

    /**
     * Get given key or next lower key if key does not exist.
     * If there is no lower key, None is returned.
     */
    pub fn floor(&self, key: &K) -> Option<&K> {
        match self.entries.binary_search_by(|k| k.key_cmp(key)) {
            Ok(index) => Some(&self.entries[index]),
            Err(index) => {
                if index > 0 {
                    Some(&self.entries[index - 1])
                } else {
                    None
                }
            }
        }
    }

    /**
     * Get given key or next higher key if key does not exist.
     * If there is no higher key, None is returned.
     */
    pub fn ceil(&self, key: &K) -> Option<&K> {
        match self.entries.binary_search_by(|k| k.key_cmp(key)) {
            Ok(index) => Some(&self.entries[index]),
            Err(index) => {
                if index < self.entries.len() {
                    Some(&self.entries[index])
                } else {
                    None
                }
            }
        }
    }

    /**
     * Get slice of keys where all keys are greater than or equal to min and less than max
     */
    pub fn sub(&self, min: &K, max: &K) -> &[K] {
        assert!(min.key_cmp(max) != Ordering::Greater);
        match (self.entries.binary_search_by(|k| k.key_cmp(min)), self.entries.binary_search_by(|k| k.key_cmp(max))) {
            (Ok(index_min), Ok(index_max)) => &self.entries[index_min..index_max],
            (Ok(index_min), Err(index_max)) => &self.entries[index_min..index_max],
            (Err(index_min), Ok(index_max)) => &self.entries[index_min..index_max],
            (Err(index_min), Err(index_max)) => &self.entries[index_min..index_max],
        }
    }

    /**
     * Retains only the elements specified by the predicate.
     *
     * In other words, remove all elements e such that f(&e) returns false.
     * This method operates in place, visiting each element exactly once in
     * the original order, and preserves the order of the retained elements.
     */
    pub fn retain<F>(&mut self, f: F)
    where
        F: Fn(&K) -> bool,
    {
        self.entries.retain(f);
    }

    /**
     * Extend this set by another set of the same keys. Values will moved from other to this
     */
    pub fn extend(&mut self, other: SortedSet<K>) {
        self.entries.extend(other.into_iter());
        self.sort();
    }

    /**
     * Makes all entries a sorted set, sorting and removing duplicates
     */
    fn sort(&mut self) {
        self.entries.sort_by(|k0, k1| k0.key_cmp(k1));
        self.entries.dedup_by(|k0, k1| k1.key_cmp(k0) == Ordering::Equal);
    }

    /**
     * Get first value in this set, panics if set is empty
     */
    #[inline]
    pub fn first(&self) -> &K {
        self.entries.first().expect("Set is empty")
    }

    /**
     * Get first value in this set, or None if set is empty
     */
    #[inline]
    pub fn try_first(&self) -> Option<&K> {
        self.entries.first()
    }

    /**
     * Get last value in this set, panics if set is empty
     */
    #[inline]
    pub fn last(&self) -> &K {
        self.entries.last().expect("Set is empty")
    }

    /**
     * Get last value in this set, or None if set is empty
     */
    #[inline]
    pub fn try_last(&self) -> Option<&K> {
        self.entries.last()
    }
}

// +++++++++++++++++++++++++++++++++++++++++++++++++++
// Map Trait Implementation

impl<K, V> ops::Deref for SortedMap<K, V>
where
    K: SortedMapKey,
{
    type Target = Vec<(K, V)>;
    fn deref(&self) -> &Vec<(K, V)> {
        &self.entries
    }
}
impl<K, V> ops::DerefMut for SortedMap<K, V>
where
    K: SortedMapKey,
{
    fn deref_mut(&mut self) -> &mut Vec<(K, V)> {
        &mut self.entries
    }
}
impl<K, V> std::convert::Into<Vec<(K, V)>> for SortedMap<K, V>
where
    K: SortedMapKey,
{
    fn into(self) -> Vec<(K, V)> {
        self.entries
    }
}
impl<K, V> std::convert::From<Vec<(K, V)>> for SortedMap<K, V>
where
    K: SortedMapKey + Send,
{
    /**
     * Convert given tuple vector to a SortedMap, using first entry as key and making all entries unique and sorted.
     * If there are duplicated keys with different values, first occurrence of key will be used.
     */
    fn from(entries: Vec<(K, V)>) -> Self {
        let mut this = SortedMap { entries };
        this.sort();
        this
    }
}
impl<K, V> std::convert::From<HashMap<K, V>> for SortedMap<K, V>
where
    K: SortedMapKey + Send,
    V: Send,
{
    fn from(entries: HashMap<K, V>) -> Self {
        entries.into_iter().collect()
    }
}
impl<K, V> std::iter::FromIterator<(K, V)> for SortedMap<K, V>
where
    K: SortedMapKey + Send,
    V: Send,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        SortedMap::from(iter.into_iter().collect::<Vec<(K, V)>>())
    }
}
impl<K, V> std::iter::IntoIterator for SortedMap<K, V>
where
    K: SortedMapKey,
{
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

// +++++++++++++++++++++++++++++++++++++++++++++++++++
// Set Trait Implementation

impl<K> ops::Deref for SortedSet<K>
where
    K: SortedMapKey,
{
    type Target = Vec<K>;
    fn deref(&self) -> &Vec<K> {
        &self.entries
    }
}
impl<K> ops::DerefMut for SortedSet<K>
where
    K: SortedMapKey,
{
    fn deref_mut(&mut self) -> &mut Vec<K> {
        &mut self.entries
    }
}
impl<K> std::convert::Into<Vec<K>> for SortedSet<K>
where
    K: SortedMapKey,
{
    fn into(self) -> Vec<K> {
        self.entries
    }
}
impl<K> std::convert::From<Vec<K>> for SortedSet<K>
where
    K: SortedMapKey,
{
    /**
     * Convert given tuple vector to a SortedSet, using first entry as key and making all entries unique and sorted.
     * If there are duplicated keys with different values, first occurrence of key will be used.
     */
    fn from(entries: Vec<K>) -> Self {
        let mut this = SortedSet { entries };
        this.sort();
        this
    }
}
impl<K> std::iter::FromIterator<K> for SortedSet<K>
where
    K: SortedMapKey + Send,
{
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        SortedSet::from(iter.into_iter().collect::<Vec<K>>())
    }
}
impl<K> std::iter::IntoIterator for SortedSet<K>
where
    K: SortedMapKey,
{
    type Item = K;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

// +++++++++++++++++++++++++++++++++++++++++++++++++++
// Key Trait Implementation

impl SortedMapKey for f64 {
    #[inline]
    fn key_cmp(&self, other: &f64) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
impl SortedMapKey for u64 {
    #[inline]
    fn key_cmp(&self, other: &u64) -> Ordering {
        self.cmp(other)
    }
}
impl SortedMapKey for usize {
    #[inline]
    fn key_cmp(&self, other: &usize) -> Ordering {
        self.cmp(other)
    }
}
impl SortedMapKey for String {
    #[inline]
    fn key_cmp(&self, other: &String) -> Ordering {
        self.cmp(other)
    }
}
impl SortedMapKey for &str {
    #[inline]
    fn key_cmp(&self, other: &&str) -> Ordering {
        self.cmp(other)
    }
}
