use hashbrown::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::iter::IntoIterator;

pub struct Strings<K, V> {
    inner: HashMap<K, V>,
}

impl<K, V> Strings<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::default(),
        }
    }
}

impl<K, V> Strings<K, V>
where
    K: Eq + Hash,
{
    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    pub fn delete<I>(&mut self, keys: I) -> usize
    where
        I: IntoIterator<Item = K>,
    {
        keys.into_iter()
            .map(|key| self.inner.remove(&key))
            .filter_map(|item| item)
            .count()
    }
}
