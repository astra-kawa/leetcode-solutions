use core::fmt;
use std::{collections::HashMap, println};

#[derive(Debug)]
struct Node {
    next: Option<i32>,
    prev: Option<i32>,
}

struct FrequencyList {
    nodes: HashMap<i32, Node>,
    most_recent: Option<i32>,
    least_recent: Option<i32>,
}

impl fmt::Debug for FrequencyList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FrequencyList: [")?;

        let mut current = self.most_recent;
        while let Some(key) = current {
            write!(f, "{}, ", key)?;
            current = self.nodes.get(&key).and_then(|node| node.prev);
        }

        write!(f, "]")?;
        write!(
            f,
            " Least: {:?} | Most: {:?}",
            self.least_recent, self.most_recent
        )?;
        Ok(())
    }
}

impl FrequencyList {
    fn new() -> Self {
        FrequencyList {
            nodes: HashMap::new(),
            most_recent: None,
            least_recent: None,
        }
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

        if let Some(most_recent) = self.most_recent
            && most_recent == key
        {
            self.most_recent = prev_key;
        }

        if let Some(least_recent) = self.least_recent
            && least_recent == key
        {
            self.least_recent = next_key;
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

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn remove_node(&mut self, key: i32) {
        self.detach(key);
        self.nodes.remove(&key);
    }

    fn remove_lru(&mut self) -> i32 {
        let least_recent = self
            .least_recent
            .expect("Least recent must exist to remove");
        self.remove_node(least_recent);

        least_recent
    }

    fn put(&mut self, key: i32) {
        if self.nodes.contains_key(&key) {
            self.detach(key);

            self.nodes.entry(key).and_modify(|node| {
                node.next = None;
                node.prev = self.most_recent;
            });
            self.add_to_front(key);
        } else {
            let node = Node {
                next: None,
                prev: self.most_recent,
            };

            self.nodes.insert(key, node);
            self.add_to_front(key);
        }
    }
}

#[derive(Debug)]
struct Entry {
    value: i32,
    counter: i32,
}

impl Entry {
    fn new(value: i32) -> Self {
        Entry { value, counter: 0 }
    }
}

#[derive(Debug)]
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
        let mut return_value = None;
        let mut new_counter = 0;

        if let Some(entry) = self.entries.get_mut(&key) {
            let map = self
                .counter_map
                .get_mut(&entry.counter)
                .expect("Counter map for should exist");

            map.remove_node(key);

            entry.counter += 1;
            new_counter = entry.counter;
            return_value = Some(entry.value);
        }

        if let Some(return_value) = return_value {
            self.add_key_to_counter_map(new_counter, key);

            let prev_counter = new_counter - 1;
            let prev_counter_map = self
                .counter_map
                .get(&prev_counter)
                .expect("List should exist for counter");

            if prev_counter_map.is_empty() {
                self.counter_map.remove(&prev_counter);
                while !self.counter_map.contains_key(&self.least_counter) {
                    self.least_counter += 1;
                }
            }

            return_value
        } else {
            -1
        }
    }

    fn add_key_to_counter_map(&mut self, counter: i32, key: i32) {
        if let Some(counter_entry) = self.counter_map.get_mut(&counter) {
            counter_entry.put(key);
        } else {
            let mut list = FrequencyList::new();
            list.put(key);

            self.counter_map.insert(counter, list);
        }
    }

    fn check_capacity(&mut self) {
        if self.entries.len() >= self.capacity as usize && self.capacity > 0 {
            while !self.counter_map.contains_key(&self.least_counter) {
                self.least_counter += 1;
            }

            let least_counter_map = self
                .counter_map
                .get_mut(&self.least_counter)
                .expect("Counter map should exist");

            let removed_key = least_counter_map.remove_lru();
            self.entries.remove(&removed_key);

            if least_counter_map.is_empty() {
                while !self.counter_map.contains_key(&self.least_counter) {
                    self.least_counter += 1;
                }
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let entry_counter;
        if let Some(entry) = self.entries.get_mut(&key) {
            self.counter_map
                .get_mut(&entry.counter)
                .expect("List should exist for counter")
                .remove_node(key);

            entry.counter += 1;
            entry.value = value;

            entry_counter = entry.counter;
        } else {
            self.check_capacity();

            let entry = Entry::new(value);
            self.entries.insert(key, entry);

            if self.least_counter != 0 {
                self.least_counter = 0;
            }

            entry_counter = 0;
        }

        self.add_key_to_counter_map(entry_counter, key);
        if entry_counter != 0 {
            let prev_counter = entry_counter - 1;
            let prev_counter_map = self
                .counter_map
                .get(&prev_counter)
                .expect("List should exist for counter");

            if prev_counter_map.is_empty() {
                self.counter_map.remove(&prev_counter);
            }
        }
    }
}

// function to remove any [ or ] from a string
fn remove_brackets(test_case: &str) -> String {
    test_case
        .chars()
        .filter(|c| *c != '[' && *c != ']')
        .collect()
}

// function to parse and run test case from leetcode
// format defines capacity first, then series of key/value pairs and gets interspersed
// format: [[<capacity>], [<key>, <value>], [<get>]] single string with no new lines, comma-separated
// example: [[2], [1, 1], [1], [2, 2], [2], [3, 3], [3], [4, 4], [4], [1], [1]]
fn parse_and_run_test_case(test_case: &str) -> Vec<i32> {
    let mut result = Vec::new();
    let ops = test_case.split("],").collect::<Vec<&str>>();

    let capacity = remove_brackets(ops.first().unwrap()).parse().unwrap();
    let mut cache = LFUCache::new(capacity);

    for op in ops.iter().skip(1) {
        let parts: Vec<&str> = op.split(',').collect();
        if parts.len() == 2 {
            let key: i32 = remove_brackets(parts[0]).parse().unwrap();
            let value: i32 = remove_brackets(parts[1]).parse().unwrap();
            cache.put(key, value);
        } else if parts.len() == 1 {
            let key: i32 = remove_brackets(parts[0]).parse().unwrap();
            result.push(cache.get(key));
        }
    }

    println!("{:#?}", cache);

    result
}

pub fn run() {
    let test_case = "[[10],[10,13],[3,17],[6,11],[10,5],[9,10],[13],[2,19],[2],[3],[5,25],[8],[9,22],[5,5],[1,30],[11],[9,12],[7],[5],[8],[9],[4,30],[9,3],[9],[10],[10],[6,14],[3,1],[3],[10,11],[8],[2,14],[1],[5],[4],[11,4],[12,24],[5,18],[13],[7,23],[8],[12],[3,27],[2,12],[5],[2,9],[13,4],[8,18],[1,7],[6],[9,29],[8,21],[5],[6,30],[1,12],[10],[4,15],[7,22],[11,26],[8,17],[9,29],[5],[3,4],[11,30],[12],[4,29],[3],[9],[6],[3,4],[1],[10],[3,29],[10,28],[1,20],[11,13],[3],[3,12],[3,8],[10,9],[3,26],[8],[7],[5],[13,17],[2,27],[11,15],[12],[9,19],[2,15],[3,16],[1],[12,17],[9,1],[6,19],[4],[5],[5],[8,1],[11,7],[5,2],[9,28],[1],[2,2],[7,4],[4,22],[7,24],[9,26],[13,28],[11,26]]";

    parse_and_run_test_case(test_case);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

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
        assert_eq!(1, cache.get(1));

        cache.put(2, 2);
        assert_eq!(2, cache.get(2));
    }

    #[test]
    fn test_example_one() {
        let mut cache = LFUCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(1, cache.get(1));

        cache.put(3, 3);
    }
}
