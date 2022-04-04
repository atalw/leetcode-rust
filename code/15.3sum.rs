use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
		// let nums = vec![2, 3, -5, 4, 0, -1, -2, 7, 0];
		// let nums = vec![0,-4,-1,-4,-2,-3,2];
		// let nums = vec![];
		let mut res: Vec<Vec<i32>> = Vec::new();

		let mut nums = nums;
		nums.sort();

		// so busy trying to figure if i could that i didn't pause to think if i should
		let idxs: Vec<i32> = (0..nums.len() as i32).collect();
		let tuples = nums.iter().zip(idxs.iter());
		// key = val, val = index
		let map: HashMap<i32, i32> = tuples.map(|(&x, &y)| (x, y)).collect();
		assert!(map.len() <= nums.len());

		let mut i = 0;
		let mut j = 0;

		if nums.len() < 3 { return vec![] }

		while i < nums.len() - 2 {
			j = i + 1;	
			while j < nums.len() {
				let complement = -(nums[i] + nums[j]);
				match map.get_key_value(&complement) {
					Some((&val, &k)) if k > j as i32 => {
						let triplet = vec![nums[i], nums[j], val];
						if !res.contains(&triplet) {
							res.push(triplet)
						}
					},
					_ => {}
				}
				j += 1;
			}

			i += 1;
		}

		res
    }
}
