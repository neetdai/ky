use std::cmp::Eq;
use std::collections::{vec_deque::Iter, VecDeque};
use std::iter::{Extend, IntoIterator};
use std::ops::Range;

pub struct List<V> {
    inner: VecDeque<V>,
}

impl<V> List<V> {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }
}

impl<V> List<V> {
    pub fn rpush<I>(&mut self, values: I) -> usize
    where
        I: IntoIterator<Item = V>,
    {
        self.inner.extend(values);
        self.inner.len()
    }

    pub fn lpush<I>(&mut self, values: I) -> usize
    where
        I: IntoIterator<Item = V>,
    {
        let mut len = self.inner.len();
        for item in values {
            len += 1;
            self.inner.push_front(item);
        }
        len
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

    pub fn lrange(&self, start: i64, stop: i64) -> Iter<'_, V> {
        let range = Self::range(self.inner.len(), start, stop);
        self.inner.range(range)
    }

    pub fn lpop(&mut self) -> Option<V> {
        self.inner.pop_front()
    }

    pub fn rpop(&mut self) -> Option<V> {
        self.inner.pop_back()
    }

    pub fn llen(&self) -> usize {
        self.inner.len()
    }
}
