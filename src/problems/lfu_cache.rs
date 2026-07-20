// https://leetcode.com/problems/lfu-cache
use std::collections::HashMap;

struct Node {
    next: Option<i32>,
    prev: Option<i32>
}

struct FrequencyList {
    nodes: HashMap<i32, Node>,
    most_recent: Option<i32>,
    least_recent: Option<i32>
}

impl FrequencyList {
    fn new() -> Self {
        FrequencyList { nodes: HashMap::new(), most_recent: None, least_recent: None }
    }

    fn detach(&mut self, key: i32) {
        let (prev_key, next_key) = {
            let entry = self.nodes.get(&key).expect("Node should exist to detach");
            (entry.prev, entry.next)
        };

        if let Some(next) = next_key {
            self.nodes
                .entry(next)
                .and_modify(|next_entry| next_entry.prev = prev_key);
        }

        if let Some(prev) = prev_key {
            self.nodes
                .entry(prev)
                .and_modify(|prev_entry| prev_entry.next = next_key);
        }
    }

    fn add_to_front(&mut self, key: i32) {
        if let Some(most_recent) = self.most_recent {
            self.nodes.entry(most_recent).and_modify(|node| {
                node.next = Some(key);
            });
        }
        self.most_recent = Some(key);

        if let Some(least_recent) = self.least_recent
            && least_recent == key
        {
            let next_key = self.nodes.get(&key).unwrap().next;
            self.least_recent = next_key;
        } else if self.least_recent.is_none() {
            self.least_recent = Some(key);
        }
    }

    fn remove_node(&mut self, key: i32) {
        self.detach(key);
        self.nodes.remove(&key);
    }

    fn put(&mut self, key: i32) {
        if self.nodes.contains_key(&key) {
            self.detach(key);
            self.add_to_front(key);
        } else {
            self.add_to_front(key);
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.nodes.contains_key(&key) {
            if let Some(most_recent) = self.most_recent
                && most_recent != key
            {
                self.detach(key);
                self.add_to_front(key);
            }

            1
        } else {
            -1
        }
    }
}

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
    counter_map: HashMap<i32, FrequencyList>,
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

    fn check_capacity(&mut self) {
        if self.entries.len() > self.capacity as usize {
            let least_counter_map = self.counter_map.get_mut(&self.least_counter).unwrap();
            least_counter_map.
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let entry_counter;
        if let Some(entry) = self.entries.get_mut(&key) {
            self.counter_map
                .get_mut(&entry.counter)
                .unwrap()
                .remove_entry(key);

            entry.counter += 1;
            entry.value = value;

            entry_counter = entry.counter;
        } else {
            let entry = Entry::new(value);
            self.entries.insert(key, entry);

            if self.least_counter != 0 {
                self.least_counter = 0;
            }

            entry_counter = 0;
        }

        self.add_key_to_counter_mape(entry_counter, key);
        self.check_capacity();
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
