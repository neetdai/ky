use hashbrown::HashMap;

pub struct List<K, V> {
    inner: HashMap<K, Vec<V>>,
}

impl<K, V> List<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::default(),
        }
    }

    pub fn lpush(&mut self, key: K, values: &mut [V]) -> usize {
        
    }
}