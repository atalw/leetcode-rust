use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // start from 0, keep adding 1, take num from array

		let mut set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());

		let mut n = 0;
		while !set.is_empty() {
			match set.take(&n) {
				Some(val) => n += 1,
				None => return n as i32
			}
		}

		return nums.len() as i32
    }
}
