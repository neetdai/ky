mod key;
mod slot;
mod value;

pub use crate::key::Key;
use crate::slot::Slot;
pub use crate::value::{Item, Value};
use collections::{List, Set, Strings};

use crc32fast;
use hashbrown::HashMap;
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use std::convert::Into;
use std::default::Default;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::{Formatter, Result as FmtResult};
use std::iter::IntoIterator;
use std::sync::Arc;

const SLOT_LEN: u32 = 32;

pub struct TypeError;

impl Display for TypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "WRONGTYPE Operation against a key holding the wrong kind of value"
        )
    }
}

impl Debug for TypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "WRONGTYPE Operation against a key holding the wrong kind of value"
        )
    }
}

impl Error for TypeError {}

#[derive(Clone)]
pub struct Database {
    slots: Arc<Vec<Slot>>,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            slots: Arc::new(
                (0..SLOT_LEN as usize)
                    .map(|_| Slot::new())
                    .collect::<Vec<Slot>>(),
            ),
        }
    }
}

impl Database {
    fn find_point(key: &[u8]) -> usize {
        (crc32fast::hash(key) % SLOT_LEN) as usize
    }

    fn read(&self, key: &String) -> RwLockReadGuard<'_, HashMap<Key, Value<Arc<String>>>> {
        let point = Self::find_point(key.as_bytes());
        (&self.slots[point]).read()
    }

    fn write(&mut self, key: &String) -> RwLockWriteGuard<'_, HashMap<Key, Value<Arc<String>>>> {
        let point = Self::find_point(key.as_bytes());
        (&self.slots[point]).write()
    }

    pub fn get<K>(&self, key: K) -> Option<Result<Arc<String>, TypeError>>
    where
        K: Into<Key>,
    {
        let key = key.into();
        let map = self.read(&key);

        map.get(&key)
            .map(|item| item.with_string().map(|value| value.get().clone()))
    }

    pub fn set<K, V>(
        &mut self,
        key: K,
        value: V,
        expire_seconds: Option<u64>,
        expire_milliseconds: Option<u128>,
    ) where
        K: Into<Key>,
        V: Into<Arc<String>>,
    {
        let key = key.into();
        let value = Strings::set(value.into());
        let value = Value::new_string(value);

        let mut map = self.write(&key);
        map.insert(key, value);
    }

    pub fn delete<I, K>(&mut self, keys: I) -> usize
    where
        I: IntoIterator<Item = K>,
        K: Into<Key>,
    {
        keys.into_iter()
            .filter_map(|key| {
                let key = key.into();
                let mut map = self.write(&key);
                map.remove(&key)
            })
            .count()
    }

    pub fn llen<K>(&self, key: K) -> Result<usize, TypeError>
    where
        K: Into<Key>,
    {
        let key = key.into();
        let map = self.read(&key);

        map.get(&key)
            .map(|item| item.with_list().map(|value| value.llen()))
            .unwrap_or(Ok(0))
    }

    pub fn lpush<K, V, I>(&mut self, key: K, list: I) -> usize
    where
        K: Into<Key>,
        I: IntoIterator<Item = V>,
        V: Into<Arc<String>>,
    {
        let key = key.into();
        let mut map = self.write(&key);

        let mut value = map
            .entry(key)
            .and_modify(|value| {
                if let Err(_) = value.with_list() {
                    value.set_list(List::new());
                }
            })
            .or_insert_with(|| Value::new_list(List::new()));

        value
            .with_list_mut()
            .unwrap()
            .lpush(list.into_iter().map(|item| item.into()))
    }

    pub fn rpush<K, V, I>(&mut self, key: K, list: I) -> usize
    where
        K: Into<Key>,
        I: IntoIterator<Item = V>,
        V: Into<Arc<String>>,
    {
        let key = key.into();
        let mut map = self.write(&key);

        let mut value = map
            .entry(key)
            .and_modify(|value| {
                if let Err(_) = value.with_list() {
                    value.set_list(List::new());
                }
            })
            .or_insert_with(|| Value::new_list(List::new()));

        value
            .with_list_mut()
            .unwrap()
            .rpush(list.into_iter().map(|item| item.into()))
    }

    pub fn lpop<K>(&mut self, key: K) -> Option<Result<Option<Arc<String>>, TypeError>>
    where
        K: Into<Key>,
    {
        let key = key.into();
        let mut map = self.write(&key);

        map.get_mut(&key)
            .map(|item| item.with_list_mut().map(|list| list.lpop()))
    }

    pub fn rpop<K>(&mut self, key: K) -> Option<Result<Option<Arc<String>>, TypeError>>
    where
        K: Into<Key>,
    {
        let key = key.into();
        let mut map = self.write(&key);

        map.get_mut(&key)
            .map(|item| item.with_list_mut().map(|list| list.rpop()))
    }

    pub fn lrange<K>(&self, key: K, start: i64, stop: i64) -> Vec<Arc<String>>
    where
        K: Into<Key>,
    {
        let key = key.into();
        let map = self.read(&key);

        map.get(&key)
            .and_then(|item| {
                item.with_list()
                    .map(|list| list.lrange(start, stop).cloned().collect())
                    .ok()
            })
            .unwrap_or(Vec::with_capacity(0))
    }

    pub fn mget<I, K>(&self, keys: I) -> Vec<Arc<String>>
    where
        I: IntoIterator<Item = K>,
        K: Into<Key>,
    {
        keys.into_iter()
            .filter_map(|key| {
                let key = key.into();
                let map = self.read(&key);
                map.get(&key)
                    .and_then(|item| item.with_string().ok().map(|value| value.get().clone()))
            })
            .collect()
    }

    pub fn mset<I, K, V>(&mut self, key_value_list: I)
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Key>,
        V: Into<Arc<String>>,
    {
        key_value_list.into_iter().for_each(|(key, value)| {
            let key = key.into();
            let value = Strings::set(value.into());
            let value = Value::new_string(value);
            let point = Self::find_point(key.as_bytes());
            let mut map = (&self.slots[point]).write();
            map.insert(key, value);
        });
    }

    pub fn sadd<I, K, V>(&mut self, key: K, members: I) -> usize
    where
        I: IntoIterator<Item = V>,
        K: Into<Key>,
        V: Into<Arc<String>>,
    {
        let key = key.into();
        let mut map = self.write(&key);

        let value = map
            .entry(key)
            .and_modify(|value| {
                if let Err(_) = value.with_set() {
                    value.set_sets(Set::new());
                }
            })
            .or_insert_with(|| Value::new_set(Set::new()));

        value
            .with_set_mut()
            .unwrap()
            .sadd(members.into_iter().map(|member| member.into()))
    }

    pub fn smembers<K>(&self, key: K) -> Vec<Arc<String>>
    where
        K: Into<Key>,
    {
        let key = key.into();
        let map = self.read(&key);

        map.get(&key)
            .and_then(|item| {
                item.with_set()
                    .map(|set| set.smembers().cloned().collect())
                    .ok()
            })
            .unwrap_or(Vec::new())
    }

    pub fn scard<K>(&self, key: K) -> usize
    where
        K: Into<Key>,
    {
        let key = key.into();
        let map = self.read(&key);

        map.get(&key)
            .and_then(|item| item.with_set().map(|set| set.scard()).ok())
            .unwrap_or(0)
    }
}
