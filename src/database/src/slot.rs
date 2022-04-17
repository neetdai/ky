use crate::key::Key;
use crate::value::Value;
use hashbrown::HashMap;
use parking_lot::RwLock;
use std::ops::Deref;
use std::sync::Arc;

pub(crate) struct Slot {
    inner: RwLock<HashMap<Key, Value<Arc<String>>>>,
}

impl Deref for Slot {
    type Target = RwLock<HashMap<Key, Value<Arc<String>>>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Slot {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::default()),
        }
    }
}
