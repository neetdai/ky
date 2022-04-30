use crate::TypeError;
use collections::{List, Set, Strings};

pub enum Item<V> {
    List(List<V>),
    String(Strings<V>),
    Sets(Set<V>),
}

pub struct Value<V> {
    item: Item<V>,
}

impl<V> Value<V> {
    pub fn new_string(value: Strings<V>) -> Self {
        Self {
            item: Item::String(value),
        }
    }

    pub fn new_list(value: List<V>) -> Self {
        Self {
            item: Item::List(value),
        }
    }

    pub fn new_set(value: Set<V>) -> Self {
        Self {
            item: Item::Sets(value),
        }
    }

    pub fn with_string(&self) -> Result<&Strings<V>, TypeError> {
        if let Item::String(ref value) = self.item {
            Ok(value)
        } else {
            Err(TypeError)
        }
    }

    pub fn with_string_mut(&mut self) -> Result<&mut Strings<V>, TypeError> {
        if let Item::String(ref mut value) = self.item {
            Ok(value)
        } else {
            Err(TypeError)
        }
    }

    pub fn with_list(&self) -> Result<&List<V>, TypeError> {
        if let Item::List(ref list) = self.item {
            Ok(list)
        } else {
            Err(TypeError)
        }
    }

    pub fn with_list_mut(&mut self) -> Result<&mut List<V>, TypeError> {
        if let Item::List(ref mut list) = self.item {
            Ok(list)
        } else {
            Err(TypeError)
        }
    }

    pub fn with_set(&self) -> Result<&Set<V>, TypeError> {
        if let Item::Sets(ref set) = self.item {
            Ok(set)
        } else {
            Err(TypeError)
        }
    }

    pub fn with_set_mut(&mut self) -> Result<&mut Set<V>, TypeError> {
        if let Item::Sets(ref mut set) = self.item {
            Ok(set)
        } else {
            Err(TypeError)
        }
    }

    pub fn set_string(&mut self, value: Strings<V>) {
        self.item = Item::String(value);
    }

    pub fn set_list(&mut self, value: List<V>) {
        self.item = Item::List(value);
    }

    pub fn set_sets(&mut self, value: Set<V>) {
        self.item = Item::Sets(value)
    }
}
