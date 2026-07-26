// https://leetcode.com/problems/kth-largest-element-in-a-stream

// kth largest stream
// now priority queue using binary heap
#[derive(Debug)]
struct KthLargest {
    k: usize,
    heap: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut sorted = nums;
        sorted.sort();

        let start_idx = sorted.len().saturating_sub(k as usize);
        let truncated = sorted.split_off(start_idx);

        Self {
            k: k as usize,
            heap: truncated,
        }
    }

    fn get_parent(&self, i: usize) -> usize {
        if i > 0 { (i - 1) / 2 } else { 0 }
    }

    fn get_left(&self, i: usize) -> usize {
        (2 * i) + 1
    }

    fn get_right(&self, i: usize) -> usize {
        (2 * i) + 2
    }

    fn heapify_down(&mut self, i: usize) {
        let l = self.get_left(i);
        let r = self.get_right(i);
        let n = self.heap.len();

        let mut s = i;
        if l < n
            && let (Some(left), Some(smallest)) = (self.heap.get(l), self.heap.get(s))
            && left <= smallest
        {
            s = l;
        }

        if r < n
            && let (Some(right), Some(smallest)) = (self.heap.get(r), self.heap.get(s))
            && right <= smallest
        {
            s = r;
        }

        if s != i {
            self.heap.swap(i, s);
            self.heapify_down(s);
        }
    }

    fn heapify_up(&mut self) {
        if self.heap.len() > 1 {
            let mut i = self.heap.len() - 1;

            while self.heap.get(self.get_parent(i)).unwrap() > self.heap.get(i).unwrap() {
                let parent = self.get_parent(i);
                self.heap.swap(i, parent);
                i = parent;
            }
        }
    }

    fn add(&mut self, num: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(num);
            self.heapify_up();
        } else if let Some(min) = self.heap.first()
            && &num > min
        {
            if let Some(old_min) = self.heap.get_mut(0) {
                *old_min = num;
            }

            self.heapify_down(0);
        }

        self.heap
            .first()
            .copied()
            .expect("Heap should have at least one element")
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
