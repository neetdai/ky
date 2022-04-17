use std::cmp::PartialEq;
use std::convert::From;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Hash, Eq)]
pub struct Key {
    inner: Arc<String>,
}

impl Deref for Key {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl PartialEq<String> for Key {
    fn eq(&self, other: &String) -> bool {
        *self.inner == *other
    }
}

impl PartialEq<Key> for Key {
    fn eq(&self, other: &Key) -> bool {
        *self.inner == *other.inner
    }
}

impl From<String> for Key {
    fn from(target: String) -> Self {
        Self {
            inner: Arc::new(target),
        }
    }
}
