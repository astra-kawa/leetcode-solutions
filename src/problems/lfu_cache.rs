// https://leetcode.com/problems/lfu-cache

use crate::problems::lru_cache::{self, LRUCache};
use std::collections::HashMap;

struct Entry {
    value: i32,
    counter: i32,
}

impl Entry {
    fn new(value: i32) -> Self {
        Entry { value, counter: 0 }
    }
}

struct LFUCache {
    capacity: i32,
    entries: HashMap<i32, Entry>,
    least_counter: i32,
    counter_map: HashMap<i32, LRUCache>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity,
            entries: HashMap::with_capacity(capacity as usize),
            least_counter: 0,
            counter_map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.counter += 1;

            //

            entry.value
        } else {
            -1
        }
    }

    fn add_key_to_counter_mape(&mut self, counter: i32, key: i32) {
        if let Some(counter_entry) = self.counter_map.get_mut(&counter) {
            counter_entry.put(counter, key);
        } else {
            let mut cache = LRUCache::new(self.capacity as usize);
            cache.put(key, counter);

            self.counter_map.insert(counter, cache);
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(entry) = self.entries.get_mut(&key) {
            self.counter_map
                .get_mut(&entry.counter)
                .unwrap()
                .remove_entry(key);

            entry.counter += 1;
            entry.value = value;
            self.add_key_to_counter_mape(entry.counter, key);
        } else {
            let entry = Entry::new(value);
            self.entries.insert(key, entry);

            if self.least_counter != 0 {
                self.least_counter = 0;
            }

            self.add_key_to_counter_mape(0, key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_item() {
        let mut cache = LFUCache::new(1);
        cache.put(1, 1);

        assert_eq!(1, cache.get(1));
    }

    #[test]
    fn test_single_overwrite() {
        let mut cache = LFUCache::new(1);
        cache.put(1, 1);
        cache.put(2, 2);

        assert_eq!(-1, cache.get(1));
        assert_eq!(2, cache.get(2));
    }

    #[test]
    fn test_counter() {
        let mut cache = LFUCache::new(1);
        cache.put(1, 1);
        cache.get(1);
        cache.get(1);

        cache.put(2, 2);

        assert_eq!(1, cache.get(1));
        assert_eq!(-1, cache.get(2));
    }
}
