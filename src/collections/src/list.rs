use hashbrown::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::mem::swap;
use std::ops::Range;
use std::collections::VecDeque;

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
    pub fn rpush(&mut self, key: K, values: &mut Vec<V>) -> usize {
        let mut values_len = values.len();
        let values_len = &mut values_len;
        self.inner
            .entry(key)
            .and_modify(|list| {
                *values_len += list.len();
                list.append(values);
            })
            .or_insert_with(|| {
                let mut list = Vec::new();
                list.append(values);
                list
            });
        *values_len
    }

    pub fn lpush(&mut self, key: K, values: &mut Vec<V>) -> usize {
        let mut values_len = values.len();
        let values_len = &mut values_len;
        self.inner
            .entry(key)
            .and_modify(|list| {
                *values_len += list.len();
                values.append(list);
            })
            .or_insert_with(|| {
                let mut list = Vec::new();
                list.append(values);
                list
            });
        *values_len
    }

    fn range(inner: &[V], mut start: isize, mut stop: isize) -> Range<usize> {
        let len = inner.len() as isize;
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
        Range { start: start as usize, end: (stop + 1) as usize }
    }

    pub fn lrange(&self, key: &K, start: isize, stop: isize) -> Option<&[V]> {
        self.inner.get(key).map(|list| {
            let range = Self::range(list, start, stop);
            &list[range]
        })
    }
}
