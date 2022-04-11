use std::collections::HashMap;

impl Solution {
	// https://youtu.be/cjWnW0hdF1Y
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
		let mut dp: HashMap<usize, i32> = HashMap::with_capacity(nums.len());

		for i in (0..nums.len()).rev() {
			for j in (i+1..nums.len()) {
				if nums[i] < nums[j] {
					match (dp.get(&i), dp.get(&j)) {
						(Some(&item), Some(&val)) => {
							dp.insert(i, item.max(1 + val));
						},
						(Some(&item), None) => {
							dp.insert(i, item.max(1 + 1));
						},
						(None, Some(&val)) => {
							dp.insert(i, 1.max(1 + val));
						},
						(None, None) => { // default to 1 since nums[idx] can at least choose itself
							dp.insert(i, 1 + 1);
						},
					}
				}
			}
		}

		// println!("{:?}", dp);
		
		match dp.iter().max_by_key(|(&x, &y)| y) {
			Some((&index, &val)) => val,
			None => 1
		}
    }

	fn lis_internal(nums: Vec<i32>, i: usize, count: i32) -> i32 {
		if i == 0 { return 1 }

		let mut res = Solution::lis_internal(nums, i - 1, count + 1);

		res
	}
}
