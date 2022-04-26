use hashbrown::HashSet;
use std::cmp::Eq;
use std::hash::Hash;
use std::iter::IntoIterator;

pub struct Set<V> {
    inner: HashSet<V>,
}

impl<V> Set<V> {
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }
}

impl<V> Set<V> where V: Hash + Eq {
    pub fn sadd<I>(&mut self, items: I) -> usize where I: IntoIterator<Item = V> {
        items.into_iter()
            .map(|item| {
                if self.inner.insert(item) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}
