// https://leetcode.com/problems/kth-largest-element-in-a-stream

const fn parent(i: usize) -> usize {
    (i - 1) / 2
}

const fn left(i: usize) -> usize {
    (2 * i) + 1
}

const fn right(i: usize) -> usize {
    (2 * i) + 2
}

// kth largest stream
// now priority queue using binary heap
#[derive(Debug)]
struct KthLargest {
    k: usize,
    heap: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k.try_into().expect("k must be non-negative");

        let mut sorted = nums;
        sorted.sort_unstable();

        let start_idx = sorted.len().saturating_sub(k);
        sorted.drain(..start_idx);

        Self { k, heap: sorted }
    }

    fn heapify_down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let mut s = i;

            if left(i) < n && self.heap[left(i)] < self.heap[s] {
                s = left(i);
            }
            if right(i) < n && self.heap[right(i)] < self.heap[s] {
                s = right(i);
            }

            if s == i {
                return;
            }

            self.heap.swap(i, s);
            i = s;
        }
    }

    fn heapify_up(&mut self, mut i: usize) {
        while i > 0 && self.heap[i] < self.heap[parent(i)] {
            self.heap.swap(i, parent(i));
            i = parent(i);
        }
    }

    fn add(&mut self, num: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(num);
            self.heapify_up(self.heap.len() - 1);
        } else if num > self.heap[0] {
            self.heap[0] = num;
            self.heapify_down(0);
        }

        self.heap[0]
    }
}

pub fn run() {
    let nums = vec![0];
    let mut kth = KthLargest::new(2, nums);

    println!("Add -1 (expect -1): {}", kth.add(-1));
    println!("Add 1 (expect 0): {}", kth.add(1));
    println!("Add -2 (expect 0): {}", kth.add(-2));
    println!("Add -4 (expect 0): {}", kth.add(-4));
    println!("Add 3 (expect 1): {}", kth.add(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut kth = KthLargest::new(2, vec![0]);
        assert_eq!(kth.add(-1), -1);
        assert_eq!(kth.add(1), 0);
        assert_eq!(kth.add(-2), 0);
        assert_eq!(kth.add(-4), 0);
        assert_eq!(kth.add(3), 1);
    }

    #[test]
    fn test_two() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }
}
