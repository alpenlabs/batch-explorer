use lru::LruCache;
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

/// Thread-safe LRU cache
pub struct Cache<K, V> {
    store: Arc<Mutex<LruCache<K, V>>>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Create a new cache with a maximum capacity
    pub fn new(max_size: usize) -> Self {
        Self {
            store: Arc::new(Mutex::new(LruCache::new(
                NonZeroUsize::new(max_size).expect("max_size must be non-zero"),
            ))),
        }
    }

    /// Insert a key-value pair into the cache
    pub fn insert(&self, key: K, value: V) {
        let mut store = self.store.lock().unwrap();
        store.put(key, value);
    }

    /// Retrieve a value by key from the cache
    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let mut store = self.store.lock().unwrap();
        store.get(key).cloned()
    }
}
