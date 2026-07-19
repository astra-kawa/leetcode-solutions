// source: https://leetcode.com/problems/lru-cache
use std::collections::HashMap;

struct Entry {
    value: i32,
    prev: Option<i32>,
    next: Option<i32>,
}

struct LRUCache {
    capacity: usize,
    entries: HashMap<i32, Entry>,
    most_recent: Option<i32>,
    least_recent: Option<i32>,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            entries: HashMap::with_capacity(capacity),
            most_recent: None,
            least_recent: None,
        }
    }

    fn evict_lru_if_needed(&mut self) {
        if self.entries.len() > self.capacity {
            let key = self
                .least_recent
                .expect("least_recent must reference an existing entry");

            let least_recent_entry = self.entries.get(&key).unwrap();
            self.least_recent = least_recent_entry.next;
            self.detach(key);
            self.entries.remove(&key);
        }
    }

    fn detach(&mut self, key: i32) {
        let (next_opt, prev_opt) = {
            let entry = self.entries.get(&key).unwrap();
            (entry.next, entry.prev)
        };

        if let Some(prev) = prev_opt {
            self.entries
                .entry(prev)
                .and_modify(|prev_entry| prev_entry.next = next_opt);
        }

        if let Some(next) = next_opt {
            self.entries
                .entry(next)
                .and_modify(|next_entry| next_entry.prev = prev_opt);
        }
    }

    fn attach_as_mru(&mut self, key: i32, entry_update: bool) {
        let old_most_recent = self.most_recent;

        if let Some(most_recent) = self.most_recent {
            self.entries.entry(most_recent).and_modify(|entry| {
                entry.next = Some(key);
            });
        }
        self.most_recent = Some(key);

        if let Some(least_recent) = self.least_recent
            && least_recent == key
        {
            let next_key = self.entries.get(&key).unwrap().next;
            self.least_recent = next_key;
        } else if self.least_recent.is_none() {
            self.least_recent = Some(key);
        }

        if entry_update {
            self.entries.entry(key).and_modify(|entry| {
                entry.next = None;
                entry.prev = old_most_recent;
            });
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(entry) = self.entries.get(&key) {
            let value = entry.value;

            if let Some(most_recent) = self.most_recent
                && most_recent != key
            {
                self.detach(key);
                self.attach_as_mru(key, true);
            }

            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.value = value;

            let most_recent = self.most_recent.expect("MRU should be defined");
            if most_recent != key {
                self.detach(key);
                self.attach_as_mru(key, true);
            }
        } else {
            let entry = Entry {
                value,
                prev: self.most_recent,
                next: None,
            };

            self.entries.insert(key, entry);

            self.attach_as_mru(key, false);
            self.evict_lru_if_needed();
        }
    }
}

pub fn run() {
    let mut cache = LRUCache::new(2);

    cache.put(1, 1);
    cache.put(2, 2);
    println!("Get key 1 (expected 1): {}", cache.get(1));

    cache.put(3, 3);
    println!("Get key 2 (expected -1): {}", cache.get(2));

    cache.put(4, 4);
    println!("Get key 1 (expected -1): {}", cache.get(1));
    println!("Get key 3 (expected 3): {}", cache.get(3));
    println!("Get key 4 (expected 4): {}", cache.get(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_item() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);

        assert_eq!(1, cache.get(1));
    }

    #[test]
    fn test_multiple_item() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);

        assert_eq!(1, cache.get(1));
        assert_eq!(2, cache.get(2));
    }

    #[test]
    fn test_capacity_clear() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);

        assert_eq!(-1, cache.get(1));
        assert_eq!(2, cache.get(2));
        assert_eq!(3, cache.get(3));
    }

    #[test]
    fn test_multiple_gets_and_puts() {
        let mut cache = LRUCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(1, cache.get(1));

        cache.put(3, 3);
        assert_eq!(-1, cache.get(2));

        cache.put(4, 4);
        assert_eq!(-1, cache.get(1));
        assert_eq!(3, cache.get(3));
        assert_eq!(4, cache.get(4));
    }

    #[test]
    fn test_simple_overwrite() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        cache.put(1, 2);

        assert_eq!(2, cache.get(1));
    }

    #[test]
    fn test_complex() {
        let mut cache = LRUCache::new(10);

        cache.put(10, 13);
        cache.put(3, 17);
        cache.put(6, 11);
        cache.put(10, 5);
        cache.put(9, 10);
        assert_eq!(-1, cache.get(13));

        cache.put(2, 19);
        assert_eq!(19, cache.get(2));
        assert_eq!(17, cache.get(3));

        cache.put(5, 25);
        assert_eq!(-1, cache.get(8));

        cache.put(9, 22);
        cache.put(5, 5);
        cache.put(1, 30);
        assert_eq!(-1, cache.get(11));
    }

    #[test]
    fn overwriting_mru_preserves_eviction_order() {
        let mut cache = LRUCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(2, 20); // 2 is already MRU
        cache.put(3, 3); // should evict 1
        cache.put(4, 4); // should evict 2

        assert_eq!(-1, cache.get(1));
        assert_eq!(-1, cache.get(2));
        assert_eq!(3, cache.get(3));
        assert_eq!(4, cache.get(4)); // currently fails
    }
}
