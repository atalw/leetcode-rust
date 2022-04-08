use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // start from 0, keep adding 1, take num from array
		
		let mut n = 0;
		let mut i = 0;
		
		// ratt lo formula wali vibes
		// this works on paper but how was it derived?
		// https://leetcode.com/problems/missing-number/discuss/69791/4-Line-Simple-Java-Bit-Manipulate-Solution-with-Explaination
		while i < nums.len() {
			// n ^= nums[i] ^ (i as i32 + 1);
			n = n ^ nums[i] ^ i as i32; 
			i += 1;
		}

		// important final xor
		n ^ i as i32

		// let mut set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());

		// let mut n = 0;
		// while !set.is_empty() {
		//     match set.take(&n) {
		//         Some(val) => n += 1,
		//         None => return n as i32
		//     }
		// }

		// return nums.len() as i32
    }
}
