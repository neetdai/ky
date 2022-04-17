use std::cmp::Eq;
use std::iter::IntoIterator;

pub struct Strings<V> {
    inner: V,
}

impl<V> Strings<V> {
    pub fn set(value: V) -> Self {
        Self { inner: value }
    }

    pub fn get(&self) -> &V {
        &self.inner
    }
}
