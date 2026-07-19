// source: https://leetcode.com/problems/lru-cache

use std::collections::HashMap;

struct LRUCache {
    capacity: i32,
    items: HashMap<i32, i32>,
    order: Vec<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            items: HashMap::with_capacity(capacity as usize),
            order: Vec::with_capacity(capacity as usize),
        }
    }

    fn check_capacity(&mut self) {
        if self.order.len() > self.capacity as usize {
            let popped = self.order.pop().unwrap();
            self.items.remove(&popped);
        }
    }

    fn update_order(&mut self, key: i32) {
        let position = self.order.iter().position(|&el| el == key).unwrap();
        self.order.remove(position);
        self.order.insert(0, key);
    }

    fn get(&mut self, key: i32) -> i32 {
        let mut value = -1;

        if let Some(existing_value) = self.items.get(&key) {
            value = *existing_value;
            self.update_order(key);
        }

        value
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(existing_value) = self.items.get(&key) {
            if existing_value != &value {
                self.items.insert(key, value);
            }

            self.update_order(key);
        } else {
            self.items.insert(key, value);
            self.order.insert(0, key);

            self.check_capacity();
        }
    }
}

struct Entry {
    value: i32,
    prev: Option<i32>,
    next: Option<i32>,
}

struct LRUCacheTwo {
    capacity: i32,
    entries: HashMap<i32, Entry>,
    most_recent: Option<i32>,
    least_recent: Option<i32>,
}

impl LRUCacheTwo {
    fn new(capacity: i32) -> Self {
        LRUCacheTwo {
            capacity,
            entries: HashMap::with_capacity(capacity as usize),
            most_recent: None,
            least_recent: None,
        }
    }

    fn check_capacity(&mut self) {
        if self.entries.len() > self.capacity as usize {
            // todo - remove unwraps
            let key = self.least_recent.unwrap();

            let least_recent_entry = self.entries.get(&key).unwrap();
            self.least_recent = least_recent_entry.next;
            self.entries.remove(&key);
        }
    }

    fn remove_entry_from_linked_list(&mut self, key: i32) {
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
                .and_modify(|next_entry| next_entry.next = prev_opt);
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let mut value = -1;

        self.entries.entry(key).and_modify(|entry| {
            entry.next = None;
            entry.prev = self.most_recent;

            value = entry.value;
            self.most_recent = Some(key);
        });

        value
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(existing_entry) = self.entries.get(&key) {
            if existing_entry.value != value {
                let entry = Entry {
                    value,
                    prev: self.most_recent,
                    next: None,
                };
                self.entries.insert(key, entry);
            }

            self.most_recent = Some(key);
        } else {
            let entry = Entry {
                value,
                prev: self.most_recent,
                next: None,
            };
            self.most_recent = Some(key);

            self.entries.insert(key, entry);
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
