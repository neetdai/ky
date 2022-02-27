use hashbrown::HashMap;
use std::cmp::Eq;
use std::collections::{vec_deque::Iter, VecDeque};
use std::hash::Hash;
use std::iter::{Extend, IntoIterator};
use std::ops::Range;

pub struct List<K, V> {
    inner: HashMap<K, VecDeque<V>>,
}

impl<K, V> List<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::default(),
        }
    }
}

impl<K, V> List<K, V>
where
    K: Eq + Hash,
{
    pub fn rpush<I>(&mut self, key: K, values: I) -> usize
    where
        I: IntoIterator<Item = V>,
    {
        match self.inner.get_key_value_mut(&key) {
            Some((_, list)) => {
                list.extend(values);
                list.len()
            }
            None => {
                let mut list = VecDeque::new();
                list.extend(values);
                let len = list.len();
                self.inner.insert(key, list);
                len
            }
        }
    }

    pub fn lpush<I>(&mut self, key: K, values: I) -> usize
    where
        I: IntoIterator<Item = V>,
    {
        match self.inner.get_key_value_mut(&key) {
            Some((_, list)) => {
                let mut len = list.len();
                for item in values {
                    len += 1;
                    list.push_front(item);
                }
                len
            }
            None => {
                let mut list = VecDeque::new();
                for item in values {
                    list.push_front(item);
                }
                let len = list.len();
                self.inner.insert(key, list);
                len
            }
        }
    }

    fn range(len: usize, mut start: i64, mut stop: i64) -> Range<usize> {
        let len = len as i64;
        if start < 0 {
            start += len;
        }
        if stop < 0 {
            stop += len;
        }
        if start >= len || stop < 0 || stop < start {
            return Range { start: 0, end: 0 };
        }
        if start < 0 {
            start = 0;
        }
        if stop >= len {
            stop = len - 1;
        }
        Range {
            start: start as usize,
            end: (stop + 1) as usize,
        }
    }

    pub fn lrange(&self, key: &K, start: i64, stop: i64) -> Option<Iter<'_, V>> {
        self.inner.get(key).map(|list| {
            let range = Self::range(list.len(), start, stop);
            list.range(range)
        })
    }

    pub fn lpop(&mut self, key: &K) -> Option<V> {
        self.inner.get_mut(key).and_then(|list| list.pop_front())
    }

    pub fn rpop(&mut self, key: &K) -> Option<V> {
        self.inner.get_mut(key).and_then(|list| list.pop_back())
    }

    pub fn llen(&self, key: &K) -> usize {
        self.inner
            .get(key)
            .map(|list| list.len())
            .unwrap_or_default()
    }
}
