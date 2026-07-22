use std::vec;

#[derive(Debug)]
struct KthLargest {
    k: i32,
    nums: Vec<i32>,
    kth_largest: Option<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut sorted = nums;
        sorted.sort();

        let start_idx = sorted.len().saturating_sub(k as usize);
        let truncated = sorted.split_off(start_idx);

        let kth_largest = truncated.first().copied();

        KthLargest {
            k,
            nums: truncated,
            kth_largest,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.nums.len() < self.k as usize
            || self.kth_largest.is_some() && val > self.kth_largest.unwrap()
        {
            self.nums.push(val);

            self.nums.sort();
            let start_idx = self.nums.len().saturating_sub(self.k as usize);
            self.nums = self.nums.split_off(start_idx);

            self.kth_largest = self.nums.first().copied();
        }

        self.kth_largest.expect("Kth largest should exist")
    }
}

pub fn run() {
    let nums = vec![7, 7, 7, 7, 8, 3];
    let mut kth = KthLargest::new(4, nums);

    println!("Add 2: {}", kth.add(2));
    println!("Add 10: {}", kth.add(10));
    println!("Add 9: {}", kth.add(9));
    println!("Add 9: {}", kth.add(9));
    println!("{kth:?}");
}
